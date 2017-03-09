use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

#[derive(Debug,Clone)]
enum PairwiseConstraint {
    Unconstrained,
    LessThan,
    GreaterThan,
    AbsDiff(u64),
}

#[derive(Debug)]
struct BoardConstraints {
    vertical: Vec<PairwiseConstraint>,  // (N-1)xN grid of vertical pair constraints
    horizontal: Vec<PairwiseConstraint>,  // Nx(N-1) grid of horizontal pair constraints
}

#[derive(Debug,Clone)]
struct Board {
    allowed: Vec<u64>,  // grid of NxN; each elem is bitmask of possibilities; assume N <= 64
}

impl BoardConstraints {
    fn create(n : usize) -> BoardConstraints {
        BoardConstraints {
            vertical: vec![PairwiseConstraint::Unconstrained; n * n],  // zero-padded; last row is empty
            horizontal: vec![PairwiseConstraint::Unconstrained; n * n]  // zero-padded; last column is empty
        }
    }
}

impl Board {
    fn create(n : usize) -> Board {
        Board {
            allowed: vec![0; n * n],
        }
    }
}

fn match_horizontal_constraint(row_field : &str) -> PairwiseConstraint {
    match row_field {
        "" => PairwiseConstraint::Unconstrained,
        "<" => PairwiseConstraint::LessThan,
        ">" => PairwiseConstraint::GreaterThan,
        _ => PairwiseConstraint::AbsDiff(u64::from_str(row_field).expect("didn't find valid horizontal constraint"))
    }
}

fn match_vertical_constraint(row_field : &str) -> PairwiseConstraint {
    match row_field {
        "" => PairwiseConstraint::Unconstrained,
        "^" => PairwiseConstraint::LessThan,
        "v" => PairwiseConstraint::GreaterThan,
        _ => PairwiseConstraint::AbsDiff(u64::from_str(row_field).expect("didn't find valid horizontal constraint"))
    }
}

fn parse_csv_board<T : BufRead>(s : T) -> Option<(Board, BoardConstraints)> {
    let mut lines_iter = s.lines();
    let first_line = lines_iter.next().expect("can't read first line").expect("can't read first line 2");
    let n = usize::from_str(&first_line).expect("can't parse dimension");
    let unconstrained_digit = (1 << (n as u64)) - 1;

    let mut board = Board::create(n);
    let mut board_constraints = BoardConstraints::create(n);
    for row in 0..(n - 1) {
        let row_offset = n * row;
        // Even rows are either numbers or horizontal constraints.
        {
            let row_string = lines_iter.next().expect("can't read line").expect("can't read line 2");
            println!("{}", row_string);
            let mut row_field_iter = row_string.split(',');
            for col in 0..(n - 1) {
                let offset = row_offset + col;
                let row_field = row_field_iter.next().expect("ran out of columns while looking for number");
                board.allowed[offset] = if row_field == "" { unconstrained_digit } else { u64::from_str(&row_field).expect("could not parse number constraint") };
                let row_field_2 = row_field_iter.next().expect("ran out of columns while looking for row constraint");
                println!("About to make horizontal constraint");
                board_constraints.horizontal[offset] = match_horizontal_constraint(row_field_2);
            }
            let row_field = row_field_iter.next().expect("ran out of columns while looking for number");
            board.allowed[row_offset + n - 1] = if row_field == "" { unconstrained_digit } else { u64::from_str(&row_field).expect("could not parse number constraint") };
        }
        // Odd rows are vertical constraints only.
        {
            let row_string = lines_iter.next().expect("can't read line").expect("can't read line 2");
            println!("{}", row_string);
            let mut row_field_iter = row_string.split(',');
            for col in 0..(n-1) {
                let offset = row_offset + col;
                let row_field = row_field_iter.next().expect("ran out of columns while looking for vertical constraint");
                println!("About to make vertical constraint");
                board_constraints.vertical[offset] = match_vertical_constraint(row_field);
                println!("made vertical constraint");
                let row_field_2 = row_field_iter.next().expect("ran out of columns while looking for row constraint");
                assert_eq!(row_field_2, "");
            }
            let row_field = row_field_iter.next().expect("ran out of columns while looking for vertical constraint");
            println!("About to make vertical constraint");
            board_constraints.vertical[row_offset + n - 1] = match_vertical_constraint(row_field);
            println!("made vertical constraint");
        }
    }
    let row_offset = n * (n - 1);
    // Even rows are either numbers or horizontal constraints.
    {
        let row_string = lines_iter.next().expect("can't read line").expect("can't read line 2");
        println!("{}", row_string);
        let mut row_field_iter = row_string.split(',');
        for col in 0..(n - 1) {
            let offset = row_offset + col;
            let row_field = row_field_iter.next().expect("ran out of columns while looking for number");
            board.allowed[offset] = if row_field == "" { unconstrained_digit } else { u64::from_str(&row_field).expect("could not parse number constraint") };
            let row_field_2 = row_field_iter.next().expect("ran out of columns while looking for row constraint");
            board_constraints.horizontal[offset] = match_horizontal_constraint(row_field_2);
        }
        let row_field = row_field_iter.next().expect("ran out of columns while looking for number");
        board.allowed[row_offset + n - 1] = if row_field == "" { unconstrained_digit } else { u64::from_str(&row_field).expect("could not parse number constraint") };
    }
    return Some((board, board_constraints))
}

fn main() {
    let f = File::open("boards/test_board_04.csv").expect("could not open file");
    let br = BufReader::new(f);
    let board = parse_csv_board(br).expect("couldn't parse game board");
    println!("{:?}", board);
}
