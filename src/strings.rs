use std::collections::HashMap;

use serde::Deserialize;

#[derive(Deserialize)]
pub struct StringInfo {
    pub id: String,
    pub text: String,
}

pub fn load_string_table(csv: impl AsRef<[u8]>) -> HashMap<String, StringInfo> {
    let mut reader = csv::Reader::from_reader(csv.as_ref());

    reader
        .deserialize()
        .map(|r| r.unwrap())
        .map(|s: StringInfo| (s.id.clone(), s))
        .collect()
}
