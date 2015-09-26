extern crate plusone;

use std::env;
use std::process;
use std::io::{self, Write};

fn main() {
    let mut status = 0;
    for arg in env::args().skip(1) {
        match plusone::increment_string(&arg) {
            Ok(val) => { writeln!(io::stdout(), "{}", val).unwrap(); },
            Err(_)  => {
                status = 2;
                writeln!(io::stderr(), "'{}' is not a number", arg).ok();
            },
        };
    }

    process::exit(status);
}
