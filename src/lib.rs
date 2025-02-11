mod types;
mod utils;
mod values;

use crate::types::parse_terminal_type;
use crate::values::parse_terminal_value;
use hashlink::LinkedHashMap;
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote};
use saphyr::Yaml;
use std::process::Command;
use syn::{
    parse::{Parse, Parser},
    parse_macro_input,
    punctuated::Punctuated,
    token, DeriveInput, Field, FieldValue, Ident,
};

#[proc_macro_attribute]
pub fn config_to_rs(args: TokenStream, input: TokenStream) -> TokenStream {
    let mut ast: DeriveInput = parse_macro_input!(input as DeriveInput);

    let args = args
        .to_string()
        .split(",")
        .map(|x| x.to_string())
        .collect::<Vec<String>>();
    let config_path = args[0].replace("\"", "");

    let out_path = std::env::current_dir().unwrap().join("out.rs");
    if let syn::Data::Struct(_) = &mut ast.data {
        let struct_name = ast.ident;
        let ast = do_the_yaml(config_path, struct_name);
        std::fs::write(out_path.clone(), ast.to_string()).unwrap();
        Command::new("rustfmt")
            .arg(out_path)
            .spawn()
            .unwrap()
            .wait()
            .unwrap();
        ast
    } else {
        panic!("`hyped_config` has to be used with structs")
    }
}

fn do_the_yaml(yaml_path: String, base_name: Ident) -> TokenStream {
    let file = std::fs::read_to_string(yaml_path).unwrap();
    let config = Yaml::load_from_str(&file).unwrap()[0].clone();
    let (types, values) = make_config(&base_name.to_string(), config);
    let res = quote! {
        #types
        #values

    };
    res.into()
}

fn make_config(key: &str, yaml: Yaml) -> (TokenStream2, TokenStream2) {
    match yaml {
        Yaml::Hash(hash) => process_hashtable(key, hash),
        // Yaml::Array(array) => process_array(key, array),
        // Yaml::String(string) => {
        //     let string = process_string(key.unwrap());
        //     todo!()
        // }
        // Yaml::Real(f) => println!("{:?}", f),
        // Yaml::Integer(i) => println!("{:?}", i),
        // Yaml::Boolean(b) => println!("{:?}", b),
        x => panic!("Yaml type not supported: {:?}", x),
    }
}

enum ConfigResult {
    SubConfig {
        types: TokenStream2,
        values: TokenStream2,
        type_entry: Field,
        value_entry: FieldValue,
    },
    Terminal {
        types: TokenStream2,
        values: TokenStream2,
    },
}

fn process_hashtable(key: &str, hash: LinkedHashMap<Yaml, Yaml>) -> (TokenStream2, TokenStream2) {
    let mut type_fields = syn::FieldsNamed {
        brace_token: token::Brace::default(),
        named: Default::default(),
    };
    let mut struct_fields = Punctuated::<FieldValue, token::Comma>::new();
    for (entry_key, value) in hash {
        let entry_key = entry_key.as_str().unwrap();
        if utils::is_terminal_type(&value) {
            let entry_type = parse_terminal_type(entry_key, value.clone());
            type_fields.named.push(entry_type);
            let value = parse_terminal_value(entry_key, value);
            struct_fields.push(value);
        } else if let Yaml::Array(_) = value {
            println!("Array");
            let x = make_config(entry_key, value);
            println!("{}\n", x.0.to_string());
            println!("{}", x.1.to_string());
            syn::ExprArray::parse.parse2(x.1.clone()).unwrap();
            println!("survived");

            type_fields
                .named
                .push(syn::Field::parse_named.parse2(x.0).unwrap());
            struct_fields.push(syn::FieldValue {
                attrs: Default::default(),
                member: syn::Member::Named(format_ident!("{}", entry_key)),
                colon_token: Some(token::Colon::default()),
                expr: syn::Expr::parse.parse2(x.1).unwrap(),
            });
        } else {
            println!("Recursive struct");
        }
    }
    let name = format_ident!("{}", key);
    let struct_type = quote! { struct #name #type_fields };
    let struct_values = quote! {
        const CONFIG: #name = #name
        {
            #struct_fields
        };
    };
    (struct_type, struct_values)
}

fn process_array(key: &str, array: Vec<Yaml>) -> (TokenStream2, TokenStream2) {
    if array.is_empty() {
        return (quote! { const array: Vec<()> }, quote! { array: vec![] });
    }
    match array[0] {
        Yaml::String(_) => {
            let array = array.iter().map(|x| {
                let x = x.as_str().unwrap();
                quote! { #x }
            });
            let name = format_ident!("{}", key);
            let len = array.len();
            return (
                quote! { #name: [&'static str; #len] },
                quote! { #name: [#(#array),*] },
            );
        }
        _ => panic!("Not supported yet"),
    };
}
