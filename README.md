An experiment to see the differences when trying to solve a simple problem, *adding one to all numbers given as arguments*, with Java and Rust.

One obvious difference is the amount of code needed:

```bash
$ tokei -s code .
--------------------------------------------------------------------------------------------------
 Language                  Files           Total          Blanks        Comments            Code
--------------------------------------------------------------------------------------------------
Java                          3              53              11               0              42
Rust                          2              33               5               0              28
```

Another one might be that Rust clearly is less wordy, denser, and *possibly* easier to grasp.

In any case, rust is doing the work *far more efficiently*. 
