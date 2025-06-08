/// TAC Values (operands)
#[derive(Debug, Clone)]
pub enum TacValue {
    Var(String),
    IntLit(i64),
    FloatLit(f64),
    StringLit(String),
}

impl TacValue {
    /// Convert TacValue to string representation
    pub fn to_string(&self) -> String {
        match self {
            TacValue::Var(name) => name.clone(),
            TacValue::IntLit(val) => val.to_string(),
            TacValue::FloatLit(val) => val.to_string(),
            TacValue::StringLit(val) => format!("\"{}\"", val),
        }
    }
}
