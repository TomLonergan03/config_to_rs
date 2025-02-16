use convert_case::{Case, Casing};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_quote, Expr, Field, FieldValue, ItemStruct};

use crate::types::Ast;

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
        Ast::UntypedHashTable {
            type_name,
            children,
            ..
        } => {
            let mut fields: Vec<Field> = vec![];
            for child in children {
                if let Ast::UntypedHashTable { key, type_name, .. } = child.as_ref() {
                    let field_name = format_ident! {"{}", key};
                    let field_type = format_ident! {"{}", type_name};
                    fields.push(parse_quote! { pub #field_name: #field_type });
                } else {
                    let field_name = format_ident! {"{}", child.get_key()};
                    let field_type = child.get_type().to_syn_type();
                    fields.push(parse_quote! { pub #field_name: #field_type });
                }
            }
            let type_name = format_ident! {"{}", type_name};
            let struct_type = parse_quote! { struct #type_name { #(#fields),* } };
            let mut types = vec![struct_type];
            for child in children {
                if let Ast::UntypedHashTable { .. } = child.as_ref() {
                    types.append(&mut recurse_types(child));
                }
            }
            types
        }
        Ast::TypedHashTable { .. } => todo!(),
        Ast::Array { .. } => vec![],
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
        Ast::UntypedHashTable { children, .. } => {
            let mut fields: Vec<FieldValue> = vec![];
            for child in children {
                let expr = recurse_struct(child);
                let field_name = format_ident! {"{}", child.get_key()};
                fields.push(parse_quote! { #field_name: #expr });
            }
            let struct_name = format_ident! {"{}", ast.get_type_name()};
            Expr::Struct(parse_quote! { #struct_name { #(#fields),* } })
        }
        Ast::TypedHashTable { .. } => todo!(),
        Ast::Array { children, .. } => {
            let values = children.iter().map(|child| recurse_struct(child));
            Expr::Array(parse_quote! { [ #(#values),* ] })
        }
        Ast::Int { value, .. } => Expr::Lit(parse_quote! { #value }),
        Ast::Bool { value, .. } => Expr::Lit(parse_quote! { #value }),
        Ast::String { value, .. } => Expr::Lit(parse_quote! { #value }),
        Ast::Float { value, .. } => Expr::Lit(parse_quote! { #value }),
    }
}
