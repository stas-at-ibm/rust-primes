use std::ops::Range;

use super::validation_error::{ValidationError, ValidationErrorKind};

pub struct SearchRange {
    numbers: Range<u64>,
    amount_of_partitions: u64,
    partitions: Vec<Range<u64>>,
}

impl SearchRange {
    pub fn new(start: u64, end: u64, partitions: u64) -> SearchRange {
        let mut result = SearchRange {
            numbers: start..end,
            amount_of_partitions: partitions,
            partitions: Vec::with_capacity(partitions as usize),
        };

        for partition in result.get_all_partitions().unwrap() {
            result.partitions.push(partition);
        }

        result
    }

    pub fn numbers(&self) -> &Range<u64> {
        &self.numbers
    }

    pub fn get_all_partitions(&self) -> Result<Vec<Range<u64>>, ValidationError> {
        (1..=self.amount_of_partitions)
            .map(|partition_nr| self.get_partition(partition_nr))
            .collect()
    }

    fn get_partition(&self, partition_nr: u64) -> Result<Range<u64>, ValidationError> {
        if partition_nr > self.amount_of_partitions {
            return Err(ValidationError::new(ValidationErrorKind::ThreadNumberError));
        }

        let step: u64 = (self.size() / self.amount_of_partitions) as u64;
        let lower_bound: u64 = step * (partition_nr - 1) + 1;

        if self.amount_of_partitions == partition_nr {
            Ok((self.numbers.start + lower_bound)..(self.numbers.start + self.size()))
        } else {
            Ok((self.numbers.start + lower_bound)..(self.numbers.start + (step * partition_nr)))
        }
    }

    pub fn size(&self) -> u64 {
        self.numbers.end - self.numbers.start + 1
    }

    pub fn partitions(&self) -> Vec<Range<u64>> {
        self.partitions.clone()
    }
}
