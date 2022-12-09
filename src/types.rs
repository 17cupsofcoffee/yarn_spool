use prost::Message;

// Generated code
include!(concat!(env!("OUT_DIR"), "/yarn.rs"));

impl Program {
    pub fn from_bytes(data: &[u8]) -> Program {
        Program::decode(data).unwrap()
    }
}
