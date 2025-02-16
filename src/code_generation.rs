use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_quote, Field, ItemStruct};

use crate::types::Ast;

impl Ast {
    pub fn to_rust(&self) -> TokenStream {
        let types = create_types(self);
        let structs = create_structs();
        quote! {
            #types
            #structs
        }
    }
}

fn create_types(ast: &Ast) -> TokenStream {
    let types = recurse_types(ast);
    quote! { #(#types)* }
}

fn recurse_types(ast: &Ast) -> Vec<ItemStruct> {
    if ast.is_almost_terminal() {
        return vec![create_terminal_type(ast)];
    }
    match ast {
        Ast::UntypedHashTable {
            key,
            type_name,
            children,
        } => {
            let fields = children.iter().map(|x| -> Field {
                let field_name = format_ident! {"{}", x.get_key()};
                let field_type = x.get_type().to_syn_type();
                parse_quote! { pub #field_name: #field_type }
            });
            let type_name = format_ident! {"{}", type_name};
            let struct_type = parse_quote! { struct #type_name { #(#fields),* } };
            let mut types = vec![struct_type];
            for child in children {
                types.append(&mut recurse_types(child));
            }
            types
        }
        Ast::TypedHashTable { .. } => todo!(),
        Ast::Array { .. } => vec![],
        x => unreachable!("Should not have gotten here: {:#?}", x),
    }
}

fn create_terminal_type(ast: &Ast) -> ItemStruct {
    match ast {
        Ast::UntypedHashTable {
            key,
            type_name,
            children,
        } => {
            let fields = children.iter().map(|x| -> Field {
                let field_name = format_ident! {"{}", x.get_key()};
                let field_type = x.get_type().to_syn_type();
                parse_quote! { pub #field_name: #field_type }
            });
            let type_name = format_ident! {"{}", type_name};
            parse_quote! { struct #type_name { #(#fields),* } }
        }
        x => {
            unreachable!("Should not have gotten here: {:#?}", ast);
        }
    }
}

fn create_structs() -> TokenStream {
    quote! {}
}
