use std::path::Path;

use prost::Message;

// Generated code
include!(concat!(env!("OUT_DIR"), "/yarn.rs"));

impl Program {
    pub fn from_file(path: impl AsRef<Path>) -> Program {
        let bytes = std::fs::read(path).unwrap();
        Program::from_bytes(&bytes)
    }

    pub fn from_bytes(data: &[u8]) -> Program {
        Program::decode(data).unwrap()
    }
}
