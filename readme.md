Mainarizumu solver
==================

This is a solver for [Mainarizumu](en.wikipedia.org/wiki/Mainarizumu) puzzles. The input and output formats are very basic and not very human-friendly.

Build
-----

With [Rust](www.rust-lang.org/en-US/) installed (I prefer the [rustup](www.rustup.rs) installer), just run:

    cargo build

Use
---

For a given board, expressed in a CSV format, the solver will repeatedly apply the puzzle constraints to whittle down the possibilities in each cell until the board no longer changes. At that point, a puzzle is either uniquely specified (solved), impossible to satisfy, or under-constrained.

    $ cargo run boards/test_board_01.csv
       Compiling mainarizumu_solve v0.1.0 (file:///Users/foton/temp/mainarizumu_solve)
        Finished debug [unoptimized + debuginfo] target(s) in 2.6 secs
         Running `target/debug/mainarizumu_solve boards/test_board_01.csv`
    boards/test_board_01.csv
    Initial board: Board { n: 2, allowed: [3, 3, 3, 3] }
    Constraints: BoardConstraints { vertical: [Unconstrained, Unconstrained, Unconstrained, Unconstrained], horizontal: [LessThan, Unconstrained, Unconstrained, Unconstrained] }
    board: Board { n: 2, allowed: [1, 2, 3, 3] }
    board: Board { n: 2, allowed: [1, 2, 2, 1] }
    board: Board { n: 2, allowed: [1, 2, 2, 1] }
    Solved! Unique solution.

Intepreting output
------------------

**Bitmasks**: The solver shows the allowed possibilities for each cell on each iteration of applying constraints. Note that the cells of the allowed array are bitmasks. For example, a value of 25 implies that 1, 4, and 5 are allowed, which you can see from where the 1s are in its binary representation:

     decimal: 25
     binary:  00011001
     values:  87654321

**Rows and columns**: The `n: 2` indicates that the size of the board is 2x2. Only square boards are supported. The grids are printed as 1D arrays, which you should interpret in row-major order. That is, it's what you'd get reading a board like English—left to right, line by line.

**Constraints**: The constraints are zero-padded to match the size of the whole board, which wastes space, but makes indexing easy. Vertical constraints apply to the constraint index and down one. Horizontal constraints apply to the constraint index and right one.

License
-------

I license this code under the [MIT License](https://en.wikipedia.org/wiki/MIT_License).
