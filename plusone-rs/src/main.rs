use std::io::{self, Write};

fn increment_and_print_args<I, T>(input: I)
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

#[cfg(not(test))]
fn main() {
    use std::env;
    use std::process;

    const SKIP_COUNT: usize = 1;
    if increment_and_print_args(env::args().skip(SKIP_COUNT)) != env::args_os().count() - SKIP_COUNT {
        process::exit(2);
    }
}


#[test]
fn test_name() {
    assert_eq!(increment_and_print_args(["5", "bar"].iter()), 1);
    assert_eq!(increment_and_print_args(["5", "6"].iter()), 2);

}
