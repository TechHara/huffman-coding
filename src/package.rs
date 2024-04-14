use std::cmp::Ordering;
use std::ops::Add;

/// a package that constitutes one or more symbols
#[derive(PartialEq, Eq, Clone)]
pub struct Package {
    count: usize,        // sum of counts among all constituents
    symbols: Vec<usize>, // constituent symbols
}

impl Package {
    pub fn new(symbol: usize, count: usize) -> Self {
        Self {
            count,
            symbols: vec![symbol],
        }
    }

    pub fn symbols(&self) -> &[usize] {
        &self.symbols
    }
}

/// compare by count
impl Ord for Package {
    fn cmp(&self, other: &Self) -> Ordering {
        self.count.cmp(&other.count)
    }
}

impl PartialOrd for Package {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Add for Package {
    type Output = Self;

    /// merge two packages into one
    fn add(mut self, rhs: Self) -> Self::Output {
        self.count += rhs.count;
        self.symbols.extend_from_slice(&rhs.symbols);
        self
    }
}
