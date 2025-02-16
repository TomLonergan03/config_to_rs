mod parse;
mod types;

use parse::parse;
use proc_macro::TokenStream;
use proc_macro2::Ident;
use saphyr::Yaml;
use std::process::Command;
use syn::{parse_macro_input, DeriveInput};

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
    let parse_tree = parse(&base_name.to_string(), config);
    println!("{:#?}", parse_tree);
    todo!()
}
