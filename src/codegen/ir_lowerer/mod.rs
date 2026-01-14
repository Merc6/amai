pub mod function_builder;

use std::collections::HashSet;
use function_builder::FunctionBuilder;
use crate::common::Operator;
use crate::vm::AmaiVM;
use crate::vm::value::Value;

pub struct IRLowerer {
    constants: HashSet<Value>,
    functions: Vec<FunctionBuilder>,
}

impl IRLowerer {
    pub fn new() -> Self {
        Self {
            constants: HashSet::new(),
            functions: Vec::new(),
        }
    }

    pub fn lower(&mut self) -> usize {
        self.functions.push(FunctionBuilder::new());

        0
    }

    pub fn prepare_vm(self) -> AmaiVM {
        let allow_large_bytecode = self.functions.iter()
            .map(|func| func.bytecode.len())
            .max()
            .unwrap_or(0) >= 65536;
        let constants = self.constants.into_iter()
            .collect::<Vec<_>>()
            .into_boxed_slice();
        let mut vm = AmaiVM::new(constants, allow_large_bytecode);
        for func in self.functions {
            vm.add_function(func.bytecode.into_boxed_slice(), func.constant_count);
        }

        vm
    }
}