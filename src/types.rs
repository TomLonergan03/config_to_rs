#[derive(Debug, Clone)]
pub enum Type {
    Empty,
    String,
    Real,
    Integer,
    Boolean,
    UntypedHashTable,
    HashTable(Box<Type>, Box<Type>),
    Array(Box<Type>),
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
        name: String,
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
}
