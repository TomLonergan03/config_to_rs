use proc_macro2::Ident;
use quote::{format_ident, quote};
use saphyr::Yaml;
use syn::{
    parse::{Parse, Parser},
    token, FieldValue,
};

pub fn parse_terminal_value(key: &str, value: Yaml) -> FieldValue {
    let key = format_ident!("{}", key);
    let res = match value {
        Yaml::String(s) => process_string_value(key, s),
        Yaml::Real(r) => process_real_value(key, r.parse().unwrap()),
        Yaml::Integer(i) => process_integer_value(key, i),
        Yaml::Boolean(b) => process_boolean_value(key, b),
        _ => panic!("Not supported yet"),
    };
    res
}

fn process_string_value(key: Ident, s: String) -> FieldValue {
    let tokens = quote! { #s };
    FieldValue {
        attrs: Default::default(),
        member: syn::Member::Named(key),
        colon_token: Some(token::Colon::default()),
        expr: syn::Expr::parse.parse2(tokens).unwrap(),
    }
}

fn process_real_value(key: Ident, r: f64) -> FieldValue {
    let tokens = quote! { #r };
    FieldValue {
        attrs: Default::default(),
        member: syn::Member::Named(key),
        colon_token: Some(token::Colon::default()),
        expr: syn::Expr::parse.parse2(tokens).unwrap(),
    }
}

fn process_integer_value(key: Ident, i: i64) -> FieldValue {
    let tokens = quote! { #i };
    FieldValue {
        attrs: Default::default(),
        member: syn::Member::Named(key),
        colon_token: Some(token::Colon::default()),
        expr: syn::Expr::parse.parse2(tokens).unwrap(),
    }
}

fn process_boolean_value(key: Ident, b: bool) -> FieldValue {
    let tokens = quote! { #b };
    FieldValue {
        attrs: Default::default(),
        member: syn::Member::Named(key),
        colon_token: Some(token::Colon::default()),
        expr: syn::Expr::parse.parse2(tokens).unwrap(),
    }
}
