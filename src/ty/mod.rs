mod list;

pub use list::ListStruct;

#[derive(Clone, Debug, PartialEq)]
pub enum Ty {
    Int(i64),         // int
    List(ListStruct), // list
}
