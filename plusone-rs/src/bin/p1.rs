extern crate plusone;

use std::env;
use std::process;

fn main() {
    const SKIP_COUNT: usize = 1;
    if plusone::increment_and_print_args(env::args().skip(SKIP_COUNT)) != env::args_os().count() - SKIP_COUNT {
        process::exit(2);
    }
}
