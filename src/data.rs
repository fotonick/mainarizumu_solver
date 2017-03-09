#[derive(Debug,Clone)]
pub enum PairwiseConstraint {
    Unconstrained,
    LessThan,
    GreaterThan,
    AbsDiff(u64),
}

#[derive(Debug)]
pub struct BoardConstraints {
    pub vertical: Vec<PairwiseConstraint>,
    pub horizontal: Vec<PairwiseConstraint>,
}

#[derive(Debug,Clone,PartialEq)]
pub struct Board {
    pub n : usize,
    pub allowed: Vec<u64>,  // grid of NxN; each elem is bitmask of possibilities; assume N <= 64
}

impl BoardConstraints {
    pub fn create(n : usize) -> BoardConstraints {
        BoardConstraints {
            vertical: vec![PairwiseConstraint::Unconstrained; n * n],  // zero-padded; last row is empty
            horizontal: vec![PairwiseConstraint::Unconstrained; n * n]  // zero-padded; last column is empty
        }
    }
}

impl Board {
    pub fn create(n : usize) -> Board {
        Board {
            n: n,
            allowed: vec![0; n * n],
        }
    }

    pub fn is_solved(&self) -> bool {
        self.allowed.iter().all(|x| x.count_ones() == 1)
    }

    pub fn is_inconsistent(&self) -> bool {
        self.allowed.iter().any(|x| x.count_ones() == 0)
    }
}