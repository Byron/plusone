#![feature(test)]
extern crate plusone;
extern crate test;

#[allow(unused_must_use)]
#[bench]
fn bench_increment_string_positive(b: &mut test::Bencher) {
    b.iter(|| {
        test::black_box(plusone::increment_string("5"));
    });
}

#[allow(unused_must_use)]
#[bench]
fn bench_increment_string_negative(b: &mut test::Bencher) {
    b.iter(|| {
        test::black_box(plusone::increment_string("-5"));
    });
}
