use std::num::ParseIntError;

pub fn increment_string<'a, T: AsRef<str> + 'a>(input: T) -> Result<isize, ParseIntError> {
    input.as_ref().parse::<isize>()
        .map(|v| v+1)
}

#[test]
fn test_name() {
    assert_eq!(increment_string("5"), Ok(6));
    assert_eq!(increment_string("-1"), Ok(0));
    assert!(increment_string("bar").is_err());
}
