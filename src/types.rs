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

    fn is_terminal(&self) -> bool {
        match self {
            Ast::Int { .. } => true,
            Ast::Bool { .. } => true,
            Ast::String { .. } => true,
            Ast::Float { .. } => true,
            Ast::Array { .. } => true,
            _ => false,
        }
    }

    pub fn is_almost_terminal(&self) -> bool {
        match self {
            Ast::UntypedHashTable { children, .. } => children.iter().all(|x| x.is_terminal()),
            Ast::TypedHashTable { children, .. } => children.iter().all(|x| x.is_terminal()),
            _ => false,
        }
    }
}
