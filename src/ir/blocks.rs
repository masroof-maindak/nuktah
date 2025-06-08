use super::instructions::TacInstr;

/// A labeled basic block of TAC instructions
#[derive(Debug)]
pub struct TacBlock {
    pub label: String,
    pub instrs: Vec<TacInstr>,
}

// #[derive(Debug, Clone)]
// pub(crate) struct LoopContext {
//     pub end_label: String,
// }