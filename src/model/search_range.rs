use std::ops::Range;

use super::validation_error::{ValidationError, ValidationErrorKind};

pub struct SearchRange {
    numbers: Range<u64>,
    partitions: u64,
}

impl SearchRange {
    pub fn new(&self, start: u64, end: u64, partitions: u64) -> SearchRange {
        SearchRange {
            numbers: start..end,
            partitions,
        }
    }

    pub fn get_all_partitions(&self) -> Result<Vec<Range<u64>>, ValidationError> {
        (1..=self.partitions)
            .map(|partition_nr| self.get_partition(partition_nr))
            .collect()
    }

    fn get_partition(&self, partition_nr: u64) -> Result<Range<u64>, ValidationError> {
        if partition_nr > self.partitions {
            return Err(ValidationError::new(ValidationErrorKind::ThreadNumberError));
        }

        let step: u64 = (self.size() / self.partitions) as u64;
        let lower_bound: u64 = step * (partition_nr - 1) + 1;

        if self.partitions == partition_nr {
            Ok((self.numbers.start + lower_bound)..(self.numbers.start + self.size()))
        } else {
            Ok((self.numbers.start + lower_bound)..(self.numbers.start + (step * partition_nr)))
        }
    }

    pub fn size(&self) -> u64 {
        self.numbers.end - self.numbers.start + 1
    }
}
