use std::num::ParseIntError;

pub fn increment_strings<I, T, F>(input: I, mut cb: F)
    -> usize
    where I: Iterator<Item=T>,
          T: AsRef<str>,
          F: FnMut(Result<usize, (&str, ParseIntError)>) {
    let mut valid: usize = 0;
    for item in input {
        cb(item.as_ref().parse::<usize>()
            .map(|v| {
                valid += 1;
                v+1
            })
            .map_err(|err| (item.as_ref(), err)));
    }
    valid
}

#[test]
fn test_name() {
    let mut count = 0;
    assert_eq!(increment_strings(["5", "bar"].iter(), |res| count += res.is_ok() as usize), 1);
    assert_eq!(count, 1);
    let mut err_count = 0;
    assert_eq!(increment_strings(["5", "6"].iter(), |res| err_count += res.is_err() as usize), 2);
    assert_eq!(err_count, 0);
}
