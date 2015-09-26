use std::io::{self, Write};

pub fn increment_and_print_args<I, T>(input: I)
    -> usize
    where I: Iterator<Item=T>,
          T: AsRef<str> {
    let mut valid: usize = 0;
    for item in input {
        let incremented_val = match item.as_ref().parse::<usize>() {
            Ok(val) => val + 1,
            Err(_) => {
                writeln!(io::stderr(), "'{}' is not a number", item.as_ref()).ok();
                continue
            }
        };
        valid += 1;
        writeln!(io::stdout(), "{}", incremented_val).unwrap();
    }
    valid
}

#[test]
fn test_name() {
    assert_eq!(increment_and_print_args(["5", "bar"].iter()), 1);
    assert_eq!(increment_and_print_args(["5", "6"].iter()), 2);
}
