/// TAC Values (operands)
#[derive(Debug, Clone)]
pub enum TacValue {
    Var(String),
    IntLit(i64),
    FloatLit(f64),
    StringLit(String),
    BoolLit(bool),
}
