use crate::model::validation_error::{ValidationError, ValidationErrorKind};
use std::ops::Range;

pub fn validate(threads_amount: u64, search_range: &Range<u64>) -> Option<ValidationError> {
    if threads_amount == 0 {
        return Some(ValidationError::new(ValidationErrorKind::ZeroThreadsError));
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

pub fn get_all_boundaries(
    threads_amount: u64,
    search_range: &mut Range<u64>,
) -> Result<Vec<Range<u64>>, ValidationError> {
    (1..=threads_amount)
        .map(|thread_nr| calculate_boundary(thread_nr, threads_amount, search_range))
        .collect()
}

fn calculate_boundary(
    thread_number: u64,
    threads_amount: u64,
    search_range: &mut Range<u64>,
) -> Result<Range<u64>, ValidationError> {
    let highest_number = search_range.end - search_range.start + 1;

    if thread_number > threads_amount {
        return Err(ValidationError::new(ValidationErrorKind::ThreadNumberError));
    }

    // todo move to validation function
    if threads_amount > highest_number {
        return Err(ValidationError::new(ValidationErrorKind::ThreadAmountError));
    }

    let step: u64 = (highest_number / threads_amount) as u64;
    let lower_bound: u64 = step * (thread_number - 1) + 1;

    if threads_amount == thread_number {
        Ok((search_range.start + lower_bound)..(search_range.start + highest_number))
    } else {
        Ok((search_range.start + lower_bound)..(search_range.start + (step * thread_number)))
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
