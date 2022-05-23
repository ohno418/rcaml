use std::fmt;

#[derive(Clone, Debug, PartialEq)]
pub struct ListStruct(pub Option<i64>, pub Option<Box<ListStruct>>);

impl ListStruct {
    pub fn new() -> Self {
        Self(None, None)
    }

    pub fn cons(&mut self, head: i64) {
        self.1 = Some(Box::new(self.clone()));
        self.0 = Some(head);
    }
}

impl From<&Vec<i64>> for ListStruct {
    fn from(list: &Vec<i64>) -> Self {
        let mut lst = ListStruct::new();
        for ele in list.iter().rev() {
            lst.cons(*ele);
        }
        lst
    }
}

impl fmt::Display for ListStruct {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[")?;
        let mut iter = self;
        let mut is_first = true;
        while let ListStruct(Some(head), Some(rest)) = iter {
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
        assert_eq!(ListStruct(None, None), ListStruct::new(),);
    }

    #[test]
    fn cons_to_list() {
        let mut lst = ListStruct::new();
        assert_eq!(ListStruct(None, None), lst,);

        lst.cons(1);
        assert_eq!(
            ListStruct(Some(1), Some(Box::new(ListStruct(None, None))),),
            lst,
        );

        lst.cons(2);
        assert_eq!(
            ListStruct(
                Some(2),
                Some(Box::new(ListStruct(
                    Some(1),
                    Some(Box::new(ListStruct(None, None))),
                ),)),
            ),
            lst,
        );
    }

    #[test]
    fn from_collection() {
        assert_eq!(ListStruct(None, None), ListStruct::from(&vec![]));
        assert_eq!(
            ListStruct(
                Some(1),
                Some(Box::new(ListStruct(
                    Some(2),
                    Some(Box::new(ListStruct(
                        Some(3),
                        Some(Box::new(ListStruct(None, None))),
                    ),)),
                ),)),
            ),
            ListStruct::from(&vec![1, 2, 3]),
        );
    }

    #[test]
    fn convert_to_string() {
        let list = ListStruct(
            Some(1),
            Some(Box::new(ListStruct(
                Some(2),
                Some(Box::new(ListStruct(
                    Some(3),
                    Some(Box::new(ListStruct(None, None))),
                ))),
            ))),
        );
        assert_eq!(list.to_string(), "[1; 2; 3]",);
    }
}
