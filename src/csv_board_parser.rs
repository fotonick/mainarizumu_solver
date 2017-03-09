use std::io::BufRead;
use std::str::FromStr;
use data::{Board, BoardConstraints, PairwiseConstraint};

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

pub fn parse_csv_board<T : BufRead>(s : T) -> Option<(Board, BoardConstraints)> {
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
