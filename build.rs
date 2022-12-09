fn main() {
    // TODO: This will rarely change, so it might be better to pre-compile it
    // and check into Git, so that people don't need protoc installed.
    prost_build::compile_protos(&["src/yarn_spinner.proto"], &["src/"]).unwrap();
}
