#[derive(Debug)]
pub enum ParseTree {
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
        name: String,
        children: Vec<Box<ParseTree>>,
    },
    Array {
        key: String,
        children: Vec<Box<ParseTree>>,
    },
}
