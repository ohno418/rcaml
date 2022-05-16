pub fn eval(input: String) -> Result<String, String> {
    Ok(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_input_as_is() {
        let input = "hello".to_string();
        let expected = "hello";
        let actual = eval(input).unwrap();
        assert_eq!(expected, actual);
    }
}
