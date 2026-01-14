use std::collections::HashSet;
use crate::vm::value::Value;
use super::module::IRModule;

#[derive(Debug, Clone)]
pub struct IRContext {
    constants: HashSet<Value>,
    modules: Vec<IRModule>,
}

impl IRContext {
    pub fn new() -> Self {
        Self {
            constants: HashSet::new(),
            modules: Vec::new(),
        }
    }

    pub fn create_module(&mut self, label: &str) -> *mut IRModule {
        self.modules.push(IRModule::new(label));
        self.modules.last_mut().unwrap() as *mut IRModule
    }

    pub fn add_constant(&mut self, constant: Value) -> usize {
        self.constants.insert(constant);
        self.constants.iter().position(|c| *c == constant).unwrap()
    }
}