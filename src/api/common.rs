use crate::model::validation_error::{ValidationError, ValidationErrorKind};
use std::ops::Range;

pub fn validate(amount_of_partitions: u64, search_range: &Range<u64>) -> Option<ValidationError> {
    let range_size = search_range.end - search_range.start + 1;

    if amount_of_partitions == 0 {
        return Some(ValidationError::new(ValidationErrorKind::ZeroThreadsError));
    } else if amount_of_partitions > range_size {
        return Some(ValidationError::new(ValidationErrorKind::ThreadAmountError));
    } else if search_range.start > search_range.end {
        return Some(ValidationError::new(
            ValidationErrorKind::SearchRangeStartErrror,
        ));
    } else if search_range.start == search_range.end {
        return Some(ValidationError::new(
            ValidationErrorKind::SearchRangeStartAndEndEqualErrror,
        ));
    }

    None
}

pub fn break_down_search_range_into_partitions(
    amount_of_partitions: u64,
    search_range: &mut Range<u64>,
) -> Result<Vec<Range<u64>>, ValidationError> {
    (1..=amount_of_partitions)
        .map(|partition_nr| calculate_partition(partition_nr, amount_of_partitions, search_range))
        .collect()
}

fn calculate_partition(
    partition_nr: u64,
    amount_of_partitions: u64,
    search_range: &mut Range<u64>,
) -> Result<Range<u64>, ValidationError> {
    let range_size = search_range.end - search_range.start + 1;

    if partition_nr > amount_of_partitions {
        return Err(ValidationError::new(ValidationErrorKind::ThreadNumberError));
    }

    let step: u64 = (range_size / amount_of_partitions) as u64;
    let lower_bound: u64 = step * (partition_nr - 1) + 1;

    if amount_of_partitions == partition_nr {
        Ok((search_range.start + lower_bound)..(search_range.start + range_size))
    } else {
        Ok((search_range.start + lower_bound)..(search_range.start + (step * partition_nr)))
    }
}

pub fn is_prime(n: u64) -> bool {
    if n == 1 {
        return false;
    }

    let first_eight_primes = [2, 3, 5, 7, 11, 13, 17, 19];
    if first_eight_primes.iter().any(|prime| *prime == n) {
        return true;
    }

    if first_eight_primes.iter().any(|prime| n % *prime == 0) {
        return false;
    }

    let upper_boundary = (n as f32).sqrt() as u64;

    (19..=upper_boundary).step_by(2).all(|num| n % num != 0)
}

#[cfg(test)]
mod tests {
    use super::is_prime;

    #[test]
    fn lib_zero_is_not_prime() {
        assert_eq!(is_prime(0), false);
    }

    #[test]
    fn lib_one_is_not_prime() {
        assert_eq!(is_prime(1), false);
    }

    #[test]
    fn lib_true_for_first_eight_primes() {
        let one_and_nine_primes = [2, 3, 5, 7, 11, 13, 17, 19];

        for prime in one_and_nine_primes {
            assert_eq!(is_prime(prime), true);
        }
    }

    #[test]
    fn lib_true_for_five_more_primes() {
        let five_more_primes = [23, 29, 31, 37, 41];

        for prime in five_more_primes {
            assert_eq!(is_prime(prime), true);
        }
    }

    #[test]
    fn lib_false_for_multiples_of_eight_primes() {
        let first_eight_primes = [2, 3, 5, 7, 11, 13, 17, 19];

        for prime in first_eight_primes {
            assert_eq!(is_prime(prime * prime), false);
        }
    }
}
