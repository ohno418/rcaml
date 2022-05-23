mod list;

pub use list::List;

#[derive(Clone, Debug, PartialEq)]
pub enum Ty {
    Int(i64),   // int
    List(List), // list
}
