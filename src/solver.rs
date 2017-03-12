use data::{Board, BoardConstraints, PairwiseConstraint};

pub fn propagate_constraints(board : &mut Board, board_constraints : &BoardConstraints) {
    propagate_unique_rowcol(board);
    propagate_horizontal_constraints(board, board_constraints);
    propagate_vertical_constraints(board, board_constraints);
}

fn propagate_unique_rowcol(mut board : &mut Board) {
    for (offset, allowed_bitmask) in board.allowed.clone().iter().enumerate() {  // FIXME: could make unsafe optimization
        if cell_is_solved(allowed_bitmask) {
            uniquify_value(&mut board, offset, allowed_bitmask);
        }
    }
}

fn propagate_horizontal_constraints(mut board : &mut Board, board_constraints : &BoardConstraints) {
    for (offset, constraint) in board_constraints.horizontal.iter().enumerate() {  // FIXME: could make unsafe optimization
        match *constraint {
            PairwiseConstraint::Unconstrained => (),
            PairwiseConstraint::LessThan => enforce_lessthan(&mut board, offset, offset + 1),
            PairwiseConstraint::GreaterThan => enforce_lessthan(&mut board, offset + 1, offset),
            PairwiseConstraint::AbsDiff(diff) => enforce_absdiff(&mut board, offset, offset + 1, diff),
        }
    }
}

fn propagate_vertical_constraints(mut board : &mut Board, board_constraints : &BoardConstraints) {
    let n = board.n;
    for (offset, constraint) in board_constraints.vertical.iter().enumerate() {  // FIXME: could make unsafe optimization
        match *constraint {
            PairwiseConstraint::Unconstrained => (),
            PairwiseConstraint::LessThan => enforce_lessthan(&mut board, offset, offset + n),
            PairwiseConstraint::GreaterThan => enforce_lessthan(&mut board, offset + n, offset),
            PairwiseConstraint::AbsDiff(diff) => enforce_absdiff(&mut board, offset, offset + n, diff),
        }
    }
}

fn cell_is_solved(x : &u64) -> bool {
    x.count_ones() == 1
}

fn uniquify_value(mut board : &mut Board, offset : usize, allowed_bitmask : &u64) {
    let kill_mask = !allowed_bitmask;
    let row = offset / board.n;
    let col = offset % board.n;

    // Row
    {
        let mut i = row * board.n;
        while i < (row + 1) * board.n {
            board.allowed[i] &= kill_mask;
            i += 1;
        }
    }
    // Column
    {
        let mut i = col;
        while i < board.allowed.len() {
            board.allowed[i] &= kill_mask;
            i += board.n;
        }
    }
    // Put it back since we blindly cleared it.
    board.allowed[offset] = *allowed_bitmask;
}

fn enforce_lessthan(mut board : &mut Board, lhs_offset : usize, rhs_offset : usize) {
    // Left: kill any >= max or rhs
    {
        let rhs = board.allowed[rhs_offset];
        let mask = if rhs.is_power_of_two() {
            (2 * rhs - 1) >> 1
        }
        else {
            (rhs.next_power_of_two() - 1) >> 1
        };
        board.allowed[lhs_offset] &= mask;
    }
    // Right: kill any <= min of lhs
    {
        let lhs = board.allowed[lhs_offset];
        if lhs != 0 {
            let mask = !((1 << (lhs.trailing_zeros() + 1)) - 1);
            board.allowed[rhs_offset] &= mask;
        }
    }
}

fn enforce_absdiff(mut board : &mut Board, lhs_offset : usize, rhs_offset : usize, diff : u64) {
    // Left
    {
        let rhs = board.allowed[rhs_offset];
        let mask = (rhs << diff) | (rhs >> diff);
        board.allowed[lhs_offset] &= mask;
    }
    // Right
    {
        let lhs = board.allowed[lhs_offset];
        let mask = (lhs << diff) | (lhs >> diff);
        board.allowed[rhs_offset] &= mask;
    }
}
