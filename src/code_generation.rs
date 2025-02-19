use convert_case::{Case, Casing};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_quote, Expr, Field, FieldValue, ItemStruct};

use crate::types::{Ast, Type};

impl Ast {
    pub fn to_rust(&self) -> TokenStream {
        let types = create_types(self);
        let value = create_struct(self);
        quote! {
            #types
            #value
        }
    }
}

fn create_types(ast: &Ast) -> TokenStream {
    let types = recurse_types(ast);
    quote! { #(#types)* }
}

fn recurse_types(ast: &Ast) -> Vec<ItemStruct> {
    match ast {
        Ast::HashTable {
            type_name,
            children,
            ..
        } => {
            let mut fields: Vec<Field> = vec![];
            for child in children {
                if let Ast::HashTable { key, type_name, .. } = child {
                    let field_name = format_ident! {"{}", key};
                    let field_type = format_ident! {"{}", type_name};
                    fields.push(parse_quote! { pub #field_name: #field_type });
                } else {
                    if child.get_key() == "type" {
                        panic!("Cannot have a YAML field named 'type' as it is a reserved keyword in Rust");
                    }
                    let field_name = format_ident! {"{}", child.get_key()};
                    let field_type = child.get_type().to_syn_type();
                    fields.push(parse_quote! { pub #field_name: #field_type });
                }
            }
            let type_name = format_ident! {"{}", type_name};
            let struct_type = parse_quote! { pub struct #type_name { #(#fields),* } };
            let mut types = vec![struct_type];
            for child in children {
                if let Ast::HashTable { .. } = child {
                    types.append(&mut recurse_types(child));
                } else if let Ast::Array {
                    children,
                    type_def: Type::Array(child_type, ..),
                    ..
                } = child
                {
                    if let Type::HashTable { .. } = child_type.as_ref() {
                        types.append(&mut recurse_types(&children[0]));
                    }
                }
            }
            types
        }
        x => unreachable!("Should not have gotten here: {:#?}", x),
    }
}

fn create_struct(ast: &Ast) -> TokenStream {
    let struct_name = format_ident! {"{}", ast.get_key().to_case(Case::Upper)};
    let typename = format_ident! {"{}", ast.get_type_name()};
    let expr = recurse_struct(ast);
    quote! { pub const #struct_name: #typename =  #expr ;}
}

fn recurse_struct(ast: &Ast) -> Expr {
    match ast {
        Ast::HashTable { children, .. } => {
            let mut fields: Vec<FieldValue> = vec![];
            for child in children {
                let expr = recurse_struct(child);
                let field_name = format_ident! {"{}", child.get_key()};
                fields.push(parse_quote! { #field_name: #expr });
            }
            let struct_name = format_ident! {"{}", ast.get_type_name()};
            Expr::Struct(parse_quote! { #struct_name { #(#fields),* } })
        }
        Ast::Array { children, .. } => {
            let values = children.iter().map(recurse_struct);
            Expr::Array(parse_quote! { [ #(#values),* ] })
        }
        Ast::Int { value, .. } => Expr::Lit(parse_quote! { #value }),
        Ast::Bool { value, .. } => Expr::Lit(parse_quote! { #value }),
        Ast::String { value, .. } => Expr::Lit(parse_quote! { #value }),
        Ast::Float { value, .. } => Expr::Lit(parse_quote! { #value }),
    }
}
