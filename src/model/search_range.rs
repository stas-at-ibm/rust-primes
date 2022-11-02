use std::ops::Range;

use super::validation_error::{ValidationError, ValidationErrorKind};

pub struct SearchRange {
    numbers: Range<u64>,
    partitions: Vec<Range<u64>>,
}

impl SearchRange {
    pub fn new(start: u64, end: u64, partitions: u64) -> SearchRange {
        SearchRange {
            numbers: start..end,
            partitions: SearchRange::get_all_partitions(start, end, partitions).unwrap(),
        }
    }

    fn get_all_partitions(
        start: u64,
        end: u64,
        amount_of_partitions: u64,
    ) -> Result<Vec<Range<u64>>, ValidationError> {
        (1..=amount_of_partitions)
            .map(|partition_nr| {
                SearchRange::get_partition(start, end, partition_nr, amount_of_partitions)
            })
            .collect()
    }

    fn get_partition(
        start: u64,
        end: u64,
        partition_nr: u64,
        amount_of_partitions: u64,
    ) -> Result<Range<u64>, ValidationError> {
        if partition_nr > amount_of_partitions {
            return Err(ValidationError::new(ValidationErrorKind::ThreadNumberError));
        }

        let size = end - start + 1;
        let step: u64 = (size / amount_of_partitions) as u64;
        let lower_bound: u64 = step * (partition_nr - 1) + 1;

        if amount_of_partitions == partition_nr {
            Ok((start + lower_bound)..(start + size))
        } else {
            Ok((start + lower_bound)..(start + (step * partition_nr)))
        }
    }

    pub fn numbers(&self) -> &Range<u64> {
        &self.numbers
    }

    pub fn partitions(&self) -> Vec<Range<u64>> {
        self.partitions.clone()
    }
}
