mod data;
mod csv_board_parser;
mod solver;

use std::env;
use std::fs::File;
use std::io::BufReader;

use csv_board_parser::parse_csv_board;
use solver::propagate_constraints;

fn main() {
    let filename = env::args().nth(1).expect("Expected CSV board file argument");
    println!("{}", filename);
    let f = File::open(filename).expect("could not open file");
    let br = BufReader::new(f);
    let (mut board, board_constraints) = parse_csv_board(br).expect("couldn't parse game board");
    println!("{:?}", board);
    loop {
        let prev_board = board.clone();  // FIXME: circular buffer
        propagate_constraints(&mut board, &board_constraints);
        if board.is_solved() {
            println!("Solved! Unique solution:\n{:?}", board);
            break;
        }
        if board.is_inconsistent() {
            println!("Inconsistent constraints! Final state:\n{:?}", board);
            break;
        }
        if board == prev_board {
            println!("Incompletely constrained. Final state:\n{:?}", board);
            break;
        }
    }
}
