extern crate itertools;
extern crate z3;

mod data;
mod csv_board_parser;
mod smt_solver;

use std::env;
use std::fs::File;
use std::io::BufReader;

use csv_board_parser::parse_csv_board;
use smt_solver::smt_solve_board;

fn main() {
    let filename = env::args().nth(1).expect("Expected CSV board file argument");
    println!("{}", filename);
    let f = File::open(filename).expect("could not open file");
    let br = BufReader::new(f);
    let board_constraints = parse_csv_board(br).expect("couldn't parse game board");
    smt_solve_board(&board_constraints);
}
