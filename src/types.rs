use quote::{format_ident, quote};
use syn::Type as SynType;

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Empty,
    String,
    Real,
    Integer,
    Boolean,
    HashTable {
        name: String,
        full_type: Vec<(String, Type)>,
    },
    Array(Box<Type>, usize),
}

impl Type {
    pub fn to_syn_type(&self) -> SynType {
        match self {
            Type::Empty => SynType::Verbatim(quote! { () }),
            Type::String => SynType::Verbatim(quote! { &'static str }),
            Type::Real => SynType::Verbatim(quote! { f64 }),
            Type::Integer => SynType::Verbatim(quote! { i64 }),
            Type::Boolean => SynType::Verbatim(quote! { bool }),
            Type::Array(child, size) => {
                let child = child.to_syn_type();
                SynType::Verbatim(quote! { [#child; #size] })
            }
            Type::HashTable { name, .. } => {
                let name = format_ident!("{}", name);
                SynType::Verbatim(quote! { #name })
            }
        }
    }
}

#[derive(Debug, Clone)]
pub enum Ast {
    Int {
        key: String,
        value: i64,
    },
    Bool {
        key: String,
        value: bool,
    },
    String {
        key: String,
        value: String,
    },
    Float {
        key: String,
        value: f64,
    },
    HashTable {
        key: String,
        type_name: String,
        type_def: Type,
        children: Vec<Ast>,
    },
    Array {
        key: String,
        type_def: Type,
        children: Vec<Ast>,
    },
}

impl Ast {
    pub fn get_key(&self) -> String {
        match self {
            Ast::Int { key, .. } => key.clone(),
            Ast::Bool { key, .. } => key.clone(),
            Ast::String { key, .. } => key.clone(),
            Ast::Float { key, .. } => key.clone(),
            Ast::HashTable { key, .. } => key.clone(),
            Ast::Array { key, .. } => key.clone(),
        }
    }

    pub fn get_type(&self) -> Type {
        match self {
            Ast::Int { .. } => Type::Integer,
            Ast::Bool { .. } => Type::Boolean,
            Ast::String { .. } => Type::String,
            Ast::Float { .. } => Type::Real,
            Ast::HashTable { type_def, .. } => type_def.clone(),
            Ast::Array { type_def, .. } => type_def.clone(),
        }
    }

    pub fn get_type_name(&self) -> String {
        if let Ast::HashTable { type_name, .. } = self {
            type_name.clone()
        } else {
            panic!("Only hash tables have type names")
        }
    }
}
