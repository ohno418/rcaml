mod list;

pub use list::List;

// value with its type
#[derive(Clone, Debug, PartialEq)]
pub enum Ty {
    Int(i64),   // int
    List(List), // list
}
