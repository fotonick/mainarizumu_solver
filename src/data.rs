#[derive(Debug,Clone)]
pub enum PairwiseConstraint {
    Unconstrained,
    LessThan,
    GreaterThan,
    AbsDiff(u64),
}

#[derive(Debug)]
pub struct BoardConstraints {
    pub n : usize,
    pub known: Vec<u64>,  // grid of NxN; 0 means unconstrained; assume N <= 64
    pub vertical: Vec<PairwiseConstraint>,
    pub horizontal: Vec<PairwiseConstraint>,
}

impl BoardConstraints {
    pub fn create(n : usize) -> BoardConstraints {
        BoardConstraints {
            n: n,
            known: vec![0; n * n],
            vertical: vec![PairwiseConstraint::Unconstrained; n * n],  // zero-padded; last row is empty
            horizontal: vec![PairwiseConstraint::Unconstrained; n * n]  // zero-padded; last column is empty
        }
    }
}
