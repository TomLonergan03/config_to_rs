use hashlink::LinkedHashMap;
use saphyr::Yaml;

use crate::types::ParseTree;

pub fn parse(key: &str, yaml: Yaml) -> ParseTree {
    match yaml {
        Yaml::Hash(hash) => parse_hashtable(key, hash),
        Yaml::Array(array) => parse_array(key, array),
        x => parse_terminal(key, x),
    }
}

fn parse_hashtable(key: &str, hash: LinkedHashMap<Yaml, Yaml>) -> ParseTree {
    let mut fields = vec![];
    for (entry_key, value) in hash {
        let entry_key = entry_key.as_str().unwrap();
        let value = parse(entry_key, value);
        fields.push(Box::new(value));
    }

    ParseTree::HashTable {
        name: key.to_string(),
        children: fields,
    }
}

fn parse_array(key: &str, values: Vec<Yaml>) -> ParseTree {
    let mut fields = vec![];
    for (i, value) in values.iter().enumerate() {
        let value = parse(&i.to_string(), value.clone());
        fields.push(Box::new(value));
    }

    ParseTree::Array {
        key: key.to_string(),
        children: fields,
    }
}

fn parse_terminal(key: &str, value: Yaml) -> ParseTree {
    match value {
        Yaml::String(s) => ParseTree::String {
            key: key.to_string(),
            value: s,
        },
        Yaml::Real(r) => ParseTree::Float {
            key: key.to_string(),
            value: r.parse().unwrap(),
        },
        Yaml::Integer(i) => ParseTree::Int {
            key: key.to_string(),
            value: i,
        },
        Yaml::Boolean(b) => ParseTree::Bool {
            key: key.to_string(),
            value: b,
        },
        _ => panic!("Not supported yet"),
    }
}
