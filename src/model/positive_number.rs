pub struct PositiveNumber {
    value: u64,
    is_prime: bool,
}

impl PositiveNumber {
    pub fn is_prime(&self) -> bool {
        self.is_prime
    }

    pub fn value(&self) -> u64 {
        self.value
    }
}
