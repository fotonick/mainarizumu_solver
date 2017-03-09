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

fn propagate_horizontal_constraints(board : &mut Board, board_constraints : &BoardConstraints) {
}

fn propagate_vertical_constraints(board : &mut Board, board_constraints : &BoardConstraints) {
}

fn cell_is_solved(x : &u64) -> bool {
    x.count_ones() == 1
}

fn uniquify_value(board : &mut Board, offset : usize, allowed_bitmask : &u64) {
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
