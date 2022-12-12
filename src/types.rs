use std::path::Path;

use prost::Message;

// Generated code
include!(concat!(env!("OUT_DIR"), "/yarn.rs"));

pub use instruction::OpCode;
pub use operand::Value;

impl Program {
    pub fn from_file(path: impl AsRef<Path>) -> Program {
        let bytes = std::fs::read(path).unwrap();
        Program::from_bytes(&bytes)
    }

    pub fn from_bytes(data: &[u8]) -> Program {
        Program::decode(data).unwrap()
    }
}

impl From<Value> for String {
    fn from(value: Value) -> Self {
        match value {
            Value::StringValue(v) => v,
            Value::BoolValue(v) => v.to_string(),
            Value::FloatValue(v) => v.to_string(),
        }
    }
}
