use amaic_core::Span;

#[derive(Clone)]
pub struct Function {
    pub bytecode: Box<[(u32, Span)]>,
}
