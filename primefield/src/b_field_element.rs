use crate::prime_field_element::PrimeFieldElement;

pub struct BFieldElement {
    value: u64,
}

impl PrimeFieldElement<42> for BFieldElement {
    fn new(value: u64) -> Self {
        Self {
            value: value % Self::P,
        }
    }

    fn value(&self) -> u64 {
        self.value
    }
}

#[cfg(test)]
mod b_field_element_tests {
    use super::*;

    #[test]
    fn new_value_test() {
        assert_eq!(2, 2, "two is two");
    }
}
