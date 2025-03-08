use std::{collections::HashMap, num::NonZeroU32, sync::Mutex};

use crate::{ast_gen::{Assing, AstNode, Block, Declaration, ForStatement, Function, IfElseStatement}, expr::{self, Expr, InlineFunction}, LocalId};

pub struct FunctionCtx<'a> {
    is_inline:bool,
    pub prev:Option<&'a Self>,
    pub sub_funcs:Vec<Self>,

    args:Vec<Local>,
    scope_depth:u32,
    local_next_id:u32,
    locals:Vec<Local>,
}

struct Local {
    scope_depth:LocalId,
    id:u32,
    name:Box<str>
}

impl<'a> FunctionCtx<'a> {
    fn new(args:&[Box<str>],is_inline:bool) -> Self {
        let mut ctx = Self{
            is_inline,
            prev:None,
            sub_funcs:vec![],

            args:vec![],
            scope_depth:0,
            local_next_id:args.len() as u32,
            locals:vec![],
        };

        for arg in args {
            ctx.args.push(Local{
                scope_depth: 0,
                id: ctx.args.len() as u32,
                name: arg.clone().into(),
            });
        }

        ctx
    }
}

impl<'a> From<&InlineFunction> for FunctionCtx<'a> {
    fn from(value: &InlineFunction) -> Self {
        Self::new(&value.args,true)
    }
}

impl<'a> From<&Function> for FunctionCtx<'a> {
    fn from(value: &Function) -> Self {
        Self::new(&value.args,false)
    }
}



impl<'a> FunctionCtx<'a> {
    fn add_local(&mut self,name:&str) {
        self.locals.push(Local{
            scope_depth:self.scope_depth,
            id:self.local_next_id,
            name:name.into(),
        });
        self.local_next_id += 1;
    }

    fn get_local_id(&self,name:&str) -> Option<LocalId> {
        if let Some(local) = self.args.iter().find(|x| x.name == name.into() ) {
            return Some(local.id);
        } 
        if let Some(local) = self.locals.iter().rev().find(|x| x.name == name.into() ) {
            return Some(local.id);
        };

        return None;
    }

    fn up_scope(&mut self) {
        self.scope_depth += 1;
    }

    fn down_scope(&mut self) {
        while let Some(local) = self.locals.last() {
            if local.scope_depth != self.scope_depth {
                break;
            }
            self.locals.pop();
            self.local_next_id -= 1;
        }

        self.scope_depth -= 1;
    }

    fn get_expr_of_ident(&self,name:&str) -> Expr {
        if let Some(id) = self.get_local_id(name) {
            return Expr::Local(id);
        }

        return Expr::Global(name.into());
    }


    pub fn compile(&mut self,block:Block,ir:&mut Vec<((),Option<LableId>)>) {
        if block.is_empty() {
            return;
        }
    }
}


#[derive(PartialEq, Eq, Hash)]
pub struct LableId(NonZeroU32);

static LABEL_NEXT_ID:Mutex<u32> = Mutex::new(0);

impl LableId {
    pub fn new() -> Self {
        let mut lock = LABEL_NEXT_ID.lock().unwrap();
        *lock += 1;
        return Self(unsafe{NonZeroU32::new_unchecked(*lock)}); 
    }
}