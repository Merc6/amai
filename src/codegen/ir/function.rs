use super::inst::IRInst;

#[derive(Debug, Clone)]
pub struct IRFunction {
    pub body: Vec<IRInst>,
    pub next_op_slot: u8,
}

impl IRFunction {
    pub fn new() -> Self {
        Self {
            body: Vec::new(),
            next_op_slot: 0,
        }
    }
}