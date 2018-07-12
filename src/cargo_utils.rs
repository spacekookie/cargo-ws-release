use std::{fs::File, io::Read};
use toml_edit::{Document, Item, Value};

use utils::VecSearch;

pub fn get_members(mut f: File) -> Vec<String> {
    let mut content = String::new();
    f.read_to_string(&mut content).unwrap();

    let doc = content
        .parse::<Document>()
        .expect("`Cargo.toml` wasn't syntactically correct...");

    match &doc["workspace"]["members"] {
        Item::Value(Value::Array(a)) => a
            .iter()
            .map(|v| match v {
                Value::String(s) => format!("{}", s),
                _ => unreachable!(),
            })
            .map(|s| s.replace("\"", "").trim().into())
            .collect::<Vec<String>>(),
        _ => unreachable!(),
    }
}

pub fn batch_load_configs(members: &Vec<String>) -> Vec<Document> {
    members
        .iter()
        .map(|m| File::open(format!("{}/Cargo.toml", m)).unwrap())
        .map(|mut f| {
            let mut s = String::new();
            f.read_to_string(&mut s).unwrap();
            s
        })
        .map(|s| {
            s.parse::<Document>()
                .expect("Failed to read member `Cargo.toml`")
        })
        .collect::<Vec<Document>>()
}

pub fn parse_config(doc: &Document, members: &Vec<String>) -> (String, Vec<String>) {
    (
        format!(
            "{}",
            match &doc["package"]["name"] {
                Item::Value(Value::String(s)) => s,
                _ => unreachable!(),
            }
        ).replace("\"", "")
            .trim()
            .into(),
        match &doc["dependencies"] {
            Item::Table(ref t) => members
                .iter()
                .filter(move |m| t.contains_key(m))
                .map(|s| s.clone())
                .collect::<Vec<String>>(),
            _ => unreachable!(),
        },
    )
}
