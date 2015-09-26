An experiment to see the differences when trying to solve a simple problem, *adding one to all numbers given as arguments*, with Java and Rust.

One obvious difference is the amount of code needed:

```bash
$ tokei -s code -e benches .
# curated output
--------------------------------------------------------------------------------------------------
Language                  Files           Total          Blanks        Comments            Code
--------------------------------------------------------------------------------------------------
Java                          3              53              11               0              42
Rust                          2              32               5               0              27
--------------------------------------------------------------------------------------------------
```

Another one might be that Rust clearly is less wordy, denser, and *possibly* easier to grasp.

In any case, rust is doing the work *far more efficiently*.

Simple benchmarking is easy to do in Rust, yet I was unable to get it to work using *JMH* in IntelliJ after an hour. It certainly doesn't mean much, except for showing that such a task is not too intuitive and easily done with my java toolchain.
Because of that, the Rust micro-benchmark stands without comparison.
