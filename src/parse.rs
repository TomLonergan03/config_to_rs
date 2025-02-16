use convert_case::{Case, Casing};
use hashlink::LinkedHashMap;
use saphyr::Yaml;

use crate::types::{Ast, Type};

impl Ast {
    pub fn from_yaml(base_name: String, yaml: Yaml) -> Ast {
        parse("", &base_name, yaml, false)
    }
}

fn parse(path: &str, key: &str, yaml: Yaml, in_array: bool) -> Ast {
    let path = if path.is_empty() {
        key.to_string()
    } else {
        format!("{} {}", path, key).to_case(Case::Pascal)
    };
    match yaml {
        Yaml::Hash(hash) => parse_hashtable(&path, key, hash, in_array),
        Yaml::Array(array) => parse_array(&path, key, array),
        x => parse_terminal(key, x),
    }
}

fn parse_hashtable(path: &str, key: &str, hash: LinkedHashMap<Yaml, Yaml>, in_array: bool) -> Ast {
    let mut fields = vec![];
    let mut types = vec![];
    for (entry_key, value) in hash {
        let entry_key = entry_key.as_str().unwrap();
        let value = parse(path, entry_key, value, in_array);
        let type_name = (path.to_string() + "_" + entry_key).to_case(Case::Pascal);
        let type_def = value.get_type();
        if in_array {
            todo!("Hash tables in arrays are not supported yet");
        } else {
            types.push((type_name.to_string(), type_def));
        }
        fields.push(Box::new(value.clone()));
    }
    if in_array {
        todo!()
    } else {
        Ast::UntypedHashTable {
            key: key.to_string(),
            type_name: path.to_string(),
            children: fields,
        }
    }
}

fn parse_array(path: &str, key: &str, values: Vec<Yaml>) -> Ast {
    let mut fields = vec![];
    for (i, value) in values.iter().enumerate() {
        let value = parse(path, &i.to_string(), value.clone(), true);
        fields.push(Box::new(value));
    }

    if fields.is_empty() {
        return Ast::Array {
            key: key.to_string(),
            types: Type::Array(Box::new(Type::Empty), 0),
            children: vec![],
        };
    }
    Ast::Array {
        key: key.to_string(),
        types: Type::Array(Box::new(fields[0].get_type()), fields.len()),
        children: fields,
    }
}

fn parse_terminal(key: &str, value: Yaml) -> Ast {
    match value {
        Yaml::String(s) => Ast::String {
            key: key.to_string(),
            value: s,
        },
        Yaml::Real(r) => Ast::Float {
            key: key.to_string(),
            value: r.parse().unwrap(),
        },
        Yaml::Integer(i) => Ast::Int {
            key: key.to_string(),
            value: i,
        },
        Yaml::Boolean(b) => Ast::Bool {
            key: key.to_string(),
            value: b,
        },
        _ => panic!("Not supported yet"),
    }
}
