use num_traits::{One, Zero};

pub trait PrimeFieldElement<const P: u64> {
    fn new(value: u64) -> Self;
    fn value(&self) -> u64;
}
