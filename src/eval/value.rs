// value with its type
#[derive(Clone, Debug, PartialEq)]
pub(crate) enum Value {
    Int(i64),   // int
    Bool(bool), // bool
    List(List), // list
    Fn,         // function
}

/* list */
use std::fmt;

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct List(pub Option<i64>, pub Option<Box<List>>);

impl List {
    pub fn new() -> Self {
        Self(None, None)
    }

    pub fn cons(&mut self, head: i64) {
        self.1 = Some(Box::new(self.clone()));
        self.0 = Some(head);
    }
}

impl From<&Vec<i64>> for List {
    fn from(list: &Vec<i64>) -> Self {
        let mut lst = List::new();
        for ele in list.iter().rev() {
            lst.cons(*ele);
        }
        lst
    }
}

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[")?;
        let mut iter = self;
        let mut is_first = true;
        while let List(Some(head), Some(rest)) = iter {
            if !is_first {
                write!(f, "; ")?;
            }
            write!(f, "{}", head)?;
            iter = rest;
            is_first = false;
        }
        write!(f, "]")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_empty_list() {
        assert_eq!(List(None, None), List::new(),);
    }

    #[test]
    fn cons_to_list() {
        let mut lst = List::new();
        assert_eq!(List(None, None), lst,);

        lst.cons(1);
        assert_eq!(List(Some(1), Some(Box::new(List(None, None))),), lst,);

        lst.cons(2);
        assert_eq!(
            List(
                Some(2),
                Some(Box::new(List(Some(1), Some(Box::new(List(None, None))),),)),
            ),
            lst,
        );
    }

    #[test]
    fn from_collection() {
        assert_eq!(List(None, None), List::from(&vec![]));
        assert_eq!(
            List(
                Some(1),
                Some(Box::new(List(
                    Some(2),
                    Some(Box::new(List(Some(3), Some(Box::new(List(None, None))),),)),
                ),)),
            ),
            List::from(&vec![1, 2, 3]),
        );
    }

    #[test]
    fn convert_to_string() {
        let list = List(
            Some(1),
            Some(Box::new(List(
                Some(2),
                Some(Box::new(List(Some(3), Some(Box::new(List(None, None)))))),
            ))),
        );
        assert_eq!(list.to_string(), "[1; 2; 3]",);
    }
}
