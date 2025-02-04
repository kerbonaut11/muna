#![allow(dead_code,unused_variables,)]

use proc_macro::TokenStream;
use quote::{quote, ToTokens,};
use proc_macro2::{Span, TokenStream as TokenStream2};
use syn::{parse::Parse, spanned::Spanned, *};

enum LuaType {
    Bool,
    Int,
    Float,
    String,
    Function,
    Table,
    Any
}


impl LuaType {
    fn from_str(name:&str) -> Result<Self> {
        let ty = if name == "bool" {LuaType::Bool}
        else if name == "i64" {LuaType::Int}
        else if name == "f64" {LuaType::Float}
        else if name == "String"   {LuaType::String}
        else if name == "Function" {LuaType::Function}
        else if name == "Table"    {LuaType::Table}
        else if name == "Value"    {LuaType::Any}
        else {return Err(Error::new(name.span(), "invalid"));};
        Ok(ty)
    }

    fn to_str(&self) -> &str {
        match self {
            Self::Bool => "bool",
            Self::Int => "int",
            Self::Float => "float",
            Self::String => "string",
            Self::Function => "function",
            Self::Table => "table",
            Self::Any => "Value",
        }
    }
}

enum Outputs {
    None,
    Single(LuaType),
    Multi(Vec<LuaType>)
}

struct FunctionWrapper {
    rust_fn:Expr,
    inputs:Vec<LuaType>,
    outputs:Outputs,
}

impl Parse for FunctionWrapper {
    fn parse(input: parse::ParseStream) -> Result<Self> {
        let rust_fn  = input.parse::<Expr>()?;
        input.parse::<Token![,]>()?;
        let func_t = input.parse::<TypeBareFn>()?;
        Ok(Self{
            rust_fn,
            inputs:parse_input_types(&func_t)?,
            outputs:parse_output_types(&func_t)?
        })
    }
}


#[proc_macro]
pub fn wrap(tokens:TokenStream) -> TokenStream {
    let func = parse_macro_input!(tokens as FunctionWrapper);
    let arg_count = func.inputs.len();
    let ret_count = match &func.outputs {
        Outputs::None => 0,
        Outputs::Single(_) => 1,
        Outputs::Multi(x) => x.len(),
    };
    quote! {{
        fn __wrap(vm:&mut LuaVm) -> LuaResult<()> {
            #func
        }

        LuaFunction::new_call_back(__wrap as *const (),#arg_count,#ret_count)
    }}.into()
}

fn parse_input_types(func_t:&TypeBareFn) -> syn::Result<Vec<LuaType>> {
    let mut lua_types = vec![];
    for arg in func_t.inputs.iter() {
        if let Type::Path(path) = &arg.ty {
            let ident = path.path.get_ident().ok_or(Error::new(path.span(), "invalid"))?;
            let name = ident.to_string();
            lua_types.push(LuaType::from_str(&name)?);
        }
    }
    Ok(lua_types)
}

fn parse_output_types(func_t:&TypeBareFn) -> syn::Result<Outputs> {
    let return_t = match func_t.output.clone() {
        ReturnType::Default => return Ok(Outputs::None),
        ReturnType::Type(_, t) => *t,
    };

    if let Type::Path(TypePath { ref path, .. }) = return_t  {
        if &path.segments.first().unwrap().ident.to_string() != "LuaResult" {
            return Err(Error::new(return_t.span(), "must return lua result"));
        }
        match &path.segments.first().unwrap().arguments  {
            PathArguments::None => return Err(Error::new(return_t.span(), "must return result")),
            PathArguments::Parenthesized(_) => return Err(Error::new(return_t.span(), "invalid return type")),
            PathArguments::AngleBracketed(AngleBracketedGenericArguments{ args, .. }) => {

                let ty =  if let GenericArgument::Type(ty) = args.first().unwrap() {
                    ty
                } else {
                    return Err(Error::new(return_t.span(), "invalid generic"));
                };

                return match ty {
                    Type::Tuple(TypeTuple{ elems, .. }) => 
                        Ok(Outputs::Multi(elems.iter()
                        .map(|ty| -> Result<LuaType> {
                            if let Type::Path(TypePath{ path,.. }) = ty {
                                let name = path.get_ident().ok_or(Error::new(ty.span(), "invalid type"))?.to_string();
                                Ok(LuaType::from_str(&name)?)
                            } else {
                                Err(Error::new(ty.span(), "invalid type"))
                            }
                        })
                        .collect::<Result<Vec<LuaType>>>()?)),

                    Type::Path(TypePath{ path,.. }) => {
                        let name = path.get_ident().ok_or(Error::new(ty.span(), "invalid type"))?.to_string();
                        if name == "()" {
                            Ok(Outputs::None)
                        } else {
                            Ok(Outputs::Single(LuaType::from_str(&name)?))
                        }
                    },
                    _ => {Err(Error::new(ty.span(), "invalid type"))}
                };
            }
        }
    }
    todo!()
}

const ARG_RESERVED:usize = 6;

impl ToTokens for FunctionWrapper {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let mut idents:Vec<Ident> = vec![];
        for (i,_) in self.inputs.iter().enumerate() {
            let ident = Ident::new(&format!("arg{}",i), Span::call_site());
            if i < ARG_RESERVED {
                tokens.extend(quote! {
                    let mut #ident = vm.regs[#i].try_into()?;
                });
            } else {
                let stack_index = i-ARG_RESERVED;
                tokens.extend(quote! {
                    let mut #ident = vm.stack[#stack_index].try_into()?;
                });
            }
            idents.push(ident);
        }

        let fn_expr = &self.rust_fn;
        tokens.extend(quote! {
            let result = #fn_expr(#(#idents),*);
            let result = result?;
        });

        match &self.outputs {
            Outputs::None => {},
            Outputs::Single(_) => tokens.extend(quote! {
                vm.regs[0] = result.into();
            }),

            Outputs::Multi(outputs) => for i in 0..outputs.len() {
                let tuple_index = Index::from(i);
                if i < ARG_RESERVED {
                    tokens.extend(quote! {
                        vm.regs[#i] = result.#tuple_index.into();
                    });
                } else {
                    let stack_index = i-ARG_RESERVED;
                    tokens.extend(quote! {
                        vm.stack[#stack_index] = result.#tuple_index.into();
                    });
                }
            }
        }

        tokens.extend(quote! {Ok(())}); 
    }
}