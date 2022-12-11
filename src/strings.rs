use std::collections::HashMap;
use std::io::Read;
use std::path::Path;

use csv::Reader;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct StringInfo {
    pub id: String,
    pub text: String,
}

pub fn read_string_table(csv: impl AsRef<[u8]>) -> HashMap<String, StringInfo> {
    let reader = csv::Reader::from_reader(csv.as_ref());

    read_string_table_inner(reader)
}

pub fn read_string_table_file(path: impl AsRef<Path>) -> HashMap<String, StringInfo> {
    let reader = csv::Reader::from_path(path).unwrap();

    read_string_table_inner(reader)
}

fn read_string_table_inner<T>(mut reader: Reader<T>) -> HashMap<String, StringInfo>
where
    T: Read,
{
    reader
        .deserialize()
        .map(|r| r.unwrap())
        .map(|s: StringInfo| (s.id.clone(), s))
        .collect()
}
