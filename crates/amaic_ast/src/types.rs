#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Int, Float, String, Bool, Unit, Unknown,
    Vector(Box<Type>), Func(Vec<Type>, Box<Type>),
}

impl Type {
    pub fn display(&self) -> String {
        match self {
            Type::Int => "int".to_string(),
            Type::Float => "float".to_string(),
            Type::String => "string".to_string(),
            Type::Bool => "bool".to_string(),
            Type::Unit => "()".to_string(),
            Type::Unknown => "{unknown}".to_string(),
            Type::Vector(ty) => format!("[{}]", ty.display()),
            Self::Func(args, return_ty) => format!(
                "$({}) -> {}",
                args.iter().map(|arg| arg.display()).collect::<Vec<_>>().join(", "),
                return_ty.display(),
            ),
        }
    }
}