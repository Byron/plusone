extern crate plusone;

use std::env;
use std::process;
use std::io::{self, Write};

fn main() {
    const SKIP_COUNT: usize = 1;
    if plusone::increment_strings(env::args().skip(SKIP_COUNT), |res| {
            match res {
                Ok(val)        => { writeln!(io::stdout(), "{}", val).unwrap(); },
                Err((arg, _ )) => { writeln!(io::stderr(), "'{}' is not a number", arg).ok(); },
            };
        }) != env::args_os().count() - SKIP_COUNT {
        process::exit(2);
    }
}
