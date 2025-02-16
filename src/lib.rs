mod code_generation;
mod parse;
mod types;

use proc_macro::TokenStream;
use saphyr::Yaml;
use std::process::Command;
use syn::{parse_macro_input, DeriveInput};
use types::Ast;

#[proc_macro_attribute]
/// Convert a config file to a Rust struct
///
/// # Arguments
/// - `file_type`: The type of the config file (currently only supports `yaml`)
/// - `config_path`: The path to the config file
///
/// # Example
/// ```rust
/// use config_to_rs::config_to_rs;
///
/// // tests/test.yaml
/// // parsing: working
/// // age: 22
/// // enabled:
/// // array_of_arrays:
/// //   - ["a", "b", "c"]
/// //   - ["d", "e", "f"]
/// // array_of_objects:
/// //   - name: "a"
/// //     age: 1
/// //   - name: "b"
/// //     age: 2
/// //   - name: "c"
/// //     age: 3
///
/// #[config_to_rs(yaml, tests/test.yaml)]    
/// pub struct Config;
///
/// assert_eq!(CONFIG.parsing, "working");
/// assert_eq!(CONFIG.age, 22i64);
/// assert_eq!(CONFIG.enabled, true);
/// assert_eq!(CONFIG.array_of_arrays, [["a", "b", "c"], ["d", "e", "f"]]);
/// for (i, obj) in CONFIG.array_of_objects.iter().enumerate() {
///     assert_eq!(obj.name, ['a', 'b', 'c'][i].to_string());
///     assert_eq!(obj.age, (i + 1) as i64);
/// }
/// ````
///
pub fn config_to_rs(args: TokenStream, input: TokenStream) -> TokenStream {
    let mut ast: DeriveInput = parse_macro_input!(input as DeriveInput);

    let args = args
        .to_string()
        .split(",")
        .map(|x| x.to_string())
        .collect::<Vec<String>>();
    if args.len() != 2 {
        panic!("`config_to_rs` must have exactly two arguments");
    }
    let file_type = args[0].replace("\"", "").replace(" ", "");
    if file_type != "yaml" {
        panic!("`config_to_rs` currently only supports yaml files");
    }
    let config_path = args[1].replace("\"", "").replace(" ", "");
    let debug = std::env::var("DEBUG").is_ok();

    if let syn::Data::Struct(_) = &mut ast.data {
        let struct_name = ast.ident.to_string();
        let ast = do_the_yaml(config_path, struct_name);
        if debug {
            let out_path = std::env::current_dir().unwrap().join("out.rs");
            std::fs::write(out_path.clone(), ast.to_string()).unwrap();
            Command::new("rustfmt")
                .arg(out_path)
                .spawn()
                .unwrap()
                .wait()
                .unwrap();
        }
        ast
    } else {
        panic!("`config_to_rs` must only be used on structs");
    }
}

fn do_the_yaml(yaml_path: String, base_name: String) -> TokenStream {
    let file = std::fs::read_to_string(yaml_path).unwrap();
    let config = Yaml::load_from_str(&file).unwrap()[0].clone();
    let parse_tree = Ast::from_yaml(base_name, config);
    let token_stream = parse_tree.to_rust();
    token_stream.into()
}
