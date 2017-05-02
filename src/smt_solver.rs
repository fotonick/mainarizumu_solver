use itertools::Itertools;
use std::str;
use std::vec::Vec;
use z3::*;

use data::{BoardConstraints, PairwiseConstraint};

fn make_vec_of_references<T>(v : &[T]) -> Vec<&T> {
    let outvec : Vec<&T> = v.iter().map(|x| x).collect();
    outvec
}

fn print_2d(v : &[u64], cols : usize) {
    for row in v.chunks(cols) {
        let bytes : Vec<_> = row.iter().map(|&x| ('0' as u8) + (x as u8)).collect();
        println!("{}", str::from_utf8(&bytes).unwrap());
    }
}

// Generate an expression that the solution is not allowed to be the one in model.
fn not_this_model<'a>(grid : &'a [Ast], model : &'a Model) -> Ast<'a> {
    let assignments : Vec<_> = grid.iter().map(|a| {
        let smt_val = model.eval(a).unwrap();
        a._eq(&smt_val)
    }).collect();
    let assign_refs = make_vec_of_references(&assignments);
    assign_refs[0].and(&assign_refs[1..]).not()
}

fn count_solutions_up_to(max_count : u64, grid : &[Ast], initial_model : &Model, solver : &Solver) -> u64 {
    let mut count : u64 = 1;  // already found one solution
    solver.assert(&not_this_model(grid, initial_model));
    let mut sat = solver.check();
    if sat { count += 1; }
    while sat && count < max_count {
        let model = solver.get_model();
        solver.assert(&not_this_model(grid, &model));
        sat = solver.check();
        if sat { count += 1; }
    }
    count
}

pub fn smt_solve_board(board_constraints : &BoardConstraints) {
    let n = board_constraints.n;
    let cfg = Config::new();
    let ctx = Context::new(&cfg);

    // Define grid of vars
    let mut grid = Vec::new();
    for i in 0..n {
        for j in 0..n {
            let name = format!("g{}{}", i, j);  // will be pretty for n <= 9
            grid.push(ctx.named_int_const(&name));
        }
    }

    // Bound vars to range 1..n
    let zero = ctx.from_u64(0);
    let en = ctx.from_u64(n as u64);
    let solver = Solver::new(&ctx);
    for v in grid.iter() {
        solver.assert(&v.gt(&zero));
        solver.assert(&v.le(&en));
    }

    // Set known values
    for (i, &val) in board_constraints.known.iter().enumerate() {
        if val != 0 {
            let smt_val = ctx.from_u64(val as u64);
            solver.assert(&grid[i]._eq(&smt_val));
        }
    }

    // Assert that rows are distinct
    for row in grid.chunks(n) {
        solver.assert(&row[0].distinct(&make_vec_of_references(&row[1..])));
    }

    // Assert that cols are distinct
    for col_start in 0..n {
        let col : Vec<_> = grid[col_start..].iter().step(n).collect();
        solver.assert(&col[0].distinct(&col[1..]));
    }

    // Assert horizontal constraints
    for (offset, constraint) in board_constraints.horizontal.iter().enumerate() {
        match *constraint {
            PairwiseConstraint::Unconstrained => (),
            PairwiseConstraint::LessThan => solver.assert(&grid[offset].lt(&grid[offset + 1])),
            PairwiseConstraint::GreaterThan => solver.assert(&grid[offset].gt(&grid[offset + 1])),
            PairwiseConstraint::AbsDiff(diff) => {
                let smt_diff = ctx.from_u64(diff);
                solver.assert(&grid[offset]._eq(&grid[offset + 1].add(&[&smt_diff])).or(
                              &[&grid[offset].add(&[&smt_diff])._eq(&grid[offset + 1])]))
            }
        }
    }

    // Assert horizontal constraints
    for (offset, constraint) in board_constraints.vertical.iter().enumerate() {
        match *constraint {
            PairwiseConstraint::Unconstrained => (),
            PairwiseConstraint::LessThan => solver.assert(&grid[offset].lt(&grid[offset + n])),
            PairwiseConstraint::GreaterThan => solver.assert(&grid[offset].gt(&grid[offset + n])),
            PairwiseConstraint::AbsDiff(diff) => {
                let smt_diff = ctx.from_u64(diff);
                solver.assert(&grid[offset]._eq(&grid[offset + n].add(&[&smt_diff])).or(
                              &[&grid[offset].add(&[&smt_diff])._eq(&grid[offset + n])]))
            }
        }
    }

    // Solve
    if solver.check() {
        // Print solution
        let model = solver.get_model();
        let vals : Vec<u64> = grid.iter().map(|v| model.eval(v).unwrap().as_u64().unwrap()).collect();
        print_2d(&vals, n);

        // Is the solution unique?
        const MAX_SOLUTIONS : u64 = 2;
        let num_solutions = count_solutions_up_to(MAX_SOLUTIONS, &grid, &model, &solver);
        if num_solutions == 1 {
            println!("Solution is unique");
        } else if num_solutions < MAX_SOLUTIONS {
            println!("There are {} solutions", num_solutions);
        } else {
            println!("There are at least {} solutions", num_solutions);
        }
    }
    else {
        println!("Unsatisfiable");
    }
}
