mod list;

pub(crate) use list::List;

// value with its type
#[derive(Clone, Debug, PartialEq)]
pub(crate) enum Ty {
    Int(i64),   // int
    Bool(bool), // bool
    List(List), // list
    Fn,         // function
}
