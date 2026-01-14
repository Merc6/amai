use super::function::IRFunction;

#[derive(Debug, Clone)]
pub struct IRModule {
    label: String,
    functions: Vec<IRFunction>,
}

impl IRModule {
    pub fn new(label: &str) -> Self {
        Self {
            label: label.to_string(),
            functions: Vec::new(),
        }
    }

    pub fn create_function(&mut self) -> *mut IRFunction {
        self.functions.push(IRFunction::new());
        self.functions.last_mut().unwrap() as *mut IRFunction
    }
}