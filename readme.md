Mainarizumu solver
==================

This is a solver for [Mainarizumu](en.wikipedia.org/wiki/Mainarizumu) puzzles. The input and output formats are very basic and not very human-friendly. The program works by translating the game rules into assertions for the [Z3 Theorem Prover](https://github.com/Z3Prover/z3), then running the solver.

Build
-----

With [Rust](www.rust-lang.org/en-US/) installed (I prefer the [rustup](www.rustup.rs) installer), just run:

    cargo build

Use
---

For a given board, expressed in a CSV format, it will show one solution but
solve for up to 10 more so that you have an indication of how unique the
solution is.

    $ cargo run boards/test_board_12.csv
        Finished dev [unoptimized + debuginfo] target(s) in 0.0 secs
         Running `target/debug/mainarizumu_solver boards/test_board_12.csv`
    boards/test_board_12.csv
    526314
    435261
    163452
    251643
    642135
    314526
    There are 4 solutions

License
-------

I license this code under the [MIT License](https://en.wikipedia.org/wiki/MIT_License).
