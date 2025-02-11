use quote::format_ident;
use quote::quote;
use saphyr::Yaml;
use syn::parse::Parser;
use syn::Field;

pub fn parse_terminal_type(key: &str, value: Yaml) -> Field {
    let res = match value {
        Yaml::String(_) => parse_string_type(key),
        Yaml::Real(_) => parse_real_type(key),
        Yaml::Integer(_) => parse_integer_type(key),
        Yaml::Boolean(_) => parse_boolean_type(key),
        _ => panic!("Not supported yet"),
    };
    res
}

fn parse_string_type(key: &str) -> Field {
    let key = format_ident!("{}", key);
    let res = Field::parse_named
        .parse2(quote! { pub #key: &'static str })
        .unwrap();
    res
}

fn parse_real_type(key: &str) -> Field {
    let key = format_ident!("{}", key);
    let res = Field::parse_named.parse2(quote! { pub #key: f64 }).unwrap();
    res
}

fn parse_integer_type(key: &str) -> Field {
    let key = format_ident!("{}", key);
    let res = Field::parse_named.parse2(quote! { pub #key: i64 }).unwrap();
    res
}

fn parse_boolean_type(key: &str) -> Field {
    let key = format_ident!("{}", key);
    let res = Field::parse_named
        .parse2(quote! { pub #key: bool })
        .unwrap();
    res
}
