pub mod core;
pub mod instructions;
pub mod values;
pub mod blocks;
pub mod codegen;

pub use core::generate_tac_blocks;
pub use codegen::generate_tac_code;