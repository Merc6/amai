#[derive(Clone)]
pub struct Function {
    pub constant_count: usize,
    pub bytecode: Box<[u32]>,
}