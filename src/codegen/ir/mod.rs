use std::collections::HashMap;

use crate::parser::ast::*;
use crate::vm::value::Value;

pub mod inst;
pub mod function;
pub mod module;
pub mod context;

use inst::{IRInst, OperandSlot};
use function::IRFunction;
use context::IRContext;

pub struct ASTCompiler {
    ctx: IRContext,
    module_name: Option<String>,
    scope: Vec<(HashMap<String, u8>, u8)>,
}

impl ASTCompiler {
    pub fn new() -> Self {
        Self {
            ctx: IRContext::new(),
            module_name: None,
            scope: vec![(HashMap::new(), 0)],
        }
    }

    pub fn compile(&mut self, ast: &ASTModule) {
        let name = ast.path.file_stem().unwrap().display().to_string();
        self.module_name = Some(name);
        let module = self.ctx
            .create_module(self.module_name.as_ref().unwrap());

        let func = unsafe { (*module).create_function() };

        for node in &ast.nodes {
            unsafe { self.compile_node(node, func) };
        }
    }

    #[allow(unsafe_op_in_unsafe_fn)]
    unsafe fn compile_node(&mut self, node: &ASTNode, func: *mut IRFunction) -> u8 {
        match &node.ty {
            ASTNodeType::IntLit(n) => {
                let const_id = self.ctx.add_constant(Value::from_int(*n)) as u16;
                let id = (*func).next_op_slot;
                (*func).body.push(IRInst::Load { dest: OperandSlot(id), const_id });
                (*func).next_op_slot += 1;
                id
            },
            ASTNodeType::FloatLit(n) => {
                let const_id = self.ctx.add_constant(Value::from_float(*n)) as u16;
                let id = (*func).next_op_slot;
                (*func).body.push(IRInst::Load { dest: OperandSlot(id), const_id });
                (*func).next_op_slot += 1;
                id
            },
            ASTNodeType::Boolean(n) => {
                let const_id = self.ctx.add_constant(Value::from_bool(*n)) as u16;
                let id = (*func).next_op_slot;
                (*func).body.push(IRInst::Load { dest: OperandSlot(id), const_id });
                (*func).next_op_slot += 1;
                id
            },
            ASTNodeType::Identifier(n) => self.scope.last_mut().unwrap().0[n],
            ASTNodeType::BinaryOp { op, lhs, rhs, op_tys } => {
                let lhs = self.compile_node(lhs, func);
                let rhs = self.compile_node(rhs, func);

                todo!()
            }
            _ => todo!(),
        }
    }
}