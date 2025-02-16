use proc_macro2::TokenStream;
use quote::quote;
use syn::Type as SynType;

#[derive(Debug, Clone)]
pub enum Type {
    Empty,
    String,
    Real,
    Integer,
    Boolean,
    UntypedHashTable,
    HashTable(Box<Type>, Box<Type>),
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
            x => panic!("No direct rust type for {:#?}", x),
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
    /**
     * A hash table that is not in an array so may have an arbitrary type
     */
    UntypedHashTable {
        key: String,
        type_name: String,
        children: Vec<Box<Ast>>,
    },
    /** A hash table that is in an array so must have the same type as all other tables in that
     * array
     */
    TypedHashTable {
        key: String,
        type_name: String,
        types: Type,
        children: Vec<Box<Ast>>,
    },
    Array {
        key: String,
        types: Type,
        children: Vec<Box<Ast>>,
    },
}

impl Ast {
    pub fn get_key(&self) -> String {
        match self {
            Ast::Int { key, .. } => key.clone(),
            Ast::Bool { key, .. } => key.clone(),
            Ast::String { key, .. } => key.clone(),
            Ast::Float { key, .. } => key.clone(),
            Ast::UntypedHashTable { key, .. } => key.clone(),
            Ast::TypedHashTable { key: name, .. } => name.clone(),
            Ast::Array { key, .. } => key.clone(),
        }
    }

    pub fn get_type(&self) -> Type {
        match self {
            Ast::Int { .. } => Type::Integer,
            Ast::Bool { .. } => Type::Boolean,
            Ast::String { .. } => Type::String,
            Ast::Float { .. } => Type::Real,
            Ast::UntypedHashTable { .. } => Type::UntypedHashTable,
            Ast::TypedHashTable { types, .. } => types.clone(),
            Ast::Array { types, .. } => types.clone(),
        }
    }

    pub fn get_type_name(&self) -> String {
        match self {
            Ast::UntypedHashTable { type_name, .. } => type_name.clone(),
            Ast::TypedHashTable { type_name, .. } => type_name.clone(),
            _ => panic!("Only hash tables have type names"),
        }
    }

    pub fn get_value(&self) -> TokenStream {
        match self {
            Ast::Int { value, .. } => quote! { #value },
            Ast::Bool { value, .. } => quote! { #value },
            Ast::String { value, .. } => quote! { #value },
            Ast::Float { value, .. } => quote! { #value },
            _ => panic!("Only terminal types have values"),
        }
    }
}
