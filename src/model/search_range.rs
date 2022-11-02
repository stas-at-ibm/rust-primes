use std::ops::Range;

use super::validation_error::{ValidationError, ValidationErrorKind};

pub struct SearchRange {
    numbers: Range<u64>,
    partitions: Vec<Range<u64>>,
}

impl SearchRange {
    pub fn new(start: u64, end: u64, partitions: u64) -> Result<SearchRange, ValidationError> {
        if let Some(err) = SearchRange::validate(start, end, partitions) {
            return Err(err);
        }

        Ok(SearchRange {
            numbers: start..end,
            partitions: SearchRange::get_all_partitions(start, end, partitions).unwrap(),
        })
    }

    fn validate(start: u64, end: u64, partitions: u64) -> Option<ValidationError> {
        let size = end - start + 1;

        if partitions == 0 {
            return Some(ValidationError::new(ValidationErrorKind::ZeroThreadsError));
        } else if partitions > size {
            return Some(ValidationError::new(ValidationErrorKind::ThreadAmountError));
        } else if start > end {
            return Some(ValidationError::new(
                ValidationErrorKind::SearchRangeStartErrror,
            ));
        } else if start == end {
            return Some(ValidationError::new(
                ValidationErrorKind::SearchRangeStartAndEndEqualErrror,
            ));
        }

        None
    }

    fn get_all_partitions(
        start: u64,
        end: u64,
        partitions: u64,
    ) -> Result<Vec<Range<u64>>, ValidationError> {
        (1..=partitions)
            .map(|partition_nr| SearchRange::get_partition(start, end, partition_nr, partitions))
            .collect()
    }

    fn get_partition(
        start: u64,
        end: u64,
        partition_nr: u64,
        partitions: u64,
    ) -> Result<Range<u64>, ValidationError> {
        if partition_nr > partitions {
            return Err(ValidationError::new(ValidationErrorKind::ThreadNumberError));
        }

        let size = end - start + 1;
        let step: u64 = (size / partitions) as u64;
        let lower_bound: u64 = step * (partition_nr - 1) + 1;

        if partitions == partition_nr {
            Ok((start + lower_bound)..(start + size))
        } else {
            Ok((start + lower_bound)..(start + (step * partition_nr)))
        }
    }

    pub fn partitions(&self) -> Vec<Range<u64>> {
        self.partitions.clone()
    }
}
