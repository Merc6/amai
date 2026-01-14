use super::function::Function;
use super::value::Value;
use std::rc::Rc;

#[derive(Clone)]
pub struct CallFrame {
    pub function: Rc<Function>,
    pub registers: [Value; 256],
    pub constant_idx_base: usize,
    pub ip: usize,
}