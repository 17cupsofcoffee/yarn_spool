mod dialogue;
mod strings;

pub use dialogue::*;
pub use strings::*;
pub use types::instruction::OpCode;
pub use types::operand::Value;
pub use types::{Instruction, Node, Operand, Program};

mod types {
    include!(concat!(env!("OUT_DIR"), "/yarn.rs"));
}
