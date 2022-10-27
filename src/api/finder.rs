use crate::model::validation_error::{ValidationError, ValidationErrorKind};

use std::{
    ops::Range,
    sync::mpsc::{self, Receiver, SendError, Sender},
    thread::{self, JoinHandle},
};

pub fn find_primes_parallel(
    threads_amount: u64,
    lower: u64,
    upper: u64,
) -> Result<Vec<(u64, bool)>, ValidationError> {
    let search_range: (u64, u64) = (lower, upper);
    // todo extract into validation function
    if threads_amount == 0 {
        return Err(ValidationError::new(ValidationErrorKind::ZeroThreadsError));
    } else if search_range.0 > search_range.1 {
        return Err(ValidationError::new(
            ValidationErrorKind::SearchRangeStartErrror,
        ));
    } else if search_range.0 == search_range.1 {
        return Err(ValidationError::new(
            ValidationErrorKind::SearchRangeStartAndEndEqualErrror,
        ));
    }

    let boundaries = get_all_boundaries(threads_amount, search_range);

    match boundaries {
        Ok(search_ranges_by_thread) => {
            let (rx, handles) = start_threads(search_ranges_by_thread);

            for handle in handles {
                if let Err(_) = handle.join() {
                    return Err(ValidationError::new(ValidationErrorKind::ThreadPanicError));
                }
            }

            match rx {
                Ok(rx) => Ok(rx.iter().collect()),
                Err(err) => Err(err),
            }
        }
        Err(err) => Err(err),
    }
}

fn get_all_boundaries(
    threads_amount: u64,
    search_range: (u64, u64),
) -> Result<Vec<(u64, u64)>, ValidationError> {
    (1..=threads_amount)
        .map(|thread_nr| calculate_one_boundary(thread_nr, threads_amount, search_range))
        .collect()
}

fn calculate_one_boundary(
    thread_number: u64,
    threads_amount: u64,
    search_range: (u64, u64),
) -> Result<(u64, u64), ValidationError> {
    let start = search_range.0;
    let end = search_range.1;
    let highest_number = end - start;

    if thread_number > threads_amount {
        return Err(ValidationError::new(ValidationErrorKind::ThreadNumberError));
    }

    if threads_amount > highest_number {
        return Err(ValidationError::new(ValidationErrorKind::ThreadAmountError));
    }

    let step: u64 = (highest_number / threads_amount) as u64;
    let lower_bound: u64 = step * (thread_number - 1) + 1;

    if threads_amount == thread_number {
        Ok((start + lower_bound, start + highest_number))
    } else {
        Ok((start + lower_bound, start + (step * thread_number)))
    }
}

fn start_threads(
    search_ranges_by_thread: Vec<(u64, u64)>,
) -> (
    Result<Receiver<(u64, bool)>, ValidationError>,
    Vec<JoinHandle<()>>,
) {
    let (tx, rx): (Sender<(u64, bool)>, Receiver<(u64, bool)>) = mpsc::channel();

    let handles: Vec<JoinHandle<()>> = search_ranges_by_thread
        .iter()
        .map(|search_range| (search_range, tx.clone()))
        .map(|search_range_and_tx| {
            worker(
                search_range_and_tx.0 .0,
                search_range_and_tx.0 .1,
                search_range_and_tx.1,
            )
        })
        .collect();

    (Ok(rx), handles)
}

fn worker(lower: u64, upper: u64, tx: Sender<(u64, bool)>) -> JoinHandle<()> {
    thread::spawn(move || {
        for num in lower..upper {
            if is_prime(num) {
                tx.send((num, true)).unwrap();
            } else {
                tx.send((num, false)).unwrap();
            }
        }
    })
}

fn RANGE_get_all_boundaries(
    threads_amount: u64,
    search_range: &Range<u64>,
) -> Result<Vec<Range<u64>>, ValidationError> {
    (1..=threads_amount)
        .map(|thread_nr| RANGE_calculate_one_boundary(thread_nr, threads_amount, search_range))
        .collect()
}

fn RANGE_calculate_one_boundary(
    thread_number: u64,
    threads_amount: u64,
    search_range: &Range<u64>,
) -> Result<Range<u64>, ValidationError> {
    let highest_number = search_range.end - search_range.start;

    if thread_number > threads_amount {
        return Err(ValidationError::new(ValidationErrorKind::ThreadNumberError));
    }

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

fn RANGE_worker(search_range: Range<u64>, tx: Sender<(u64, bool)>) -> JoinHandle<()> {
    thread::spawn(move || {
        for num in search_range {
            if is_prime(num) {
                tx.send((num, true)).unwrap();
            } else {
                tx.send((num, false)).unwrap();
            }
        }
    })
}

fn XXX_worker(
    nr: u64,
    lower: u64,
    upper: u64,
    tx: Sender<(u64, bool)>,
) -> Result<JoinHandle<Result<(), SendError<(u64, bool)>>>, std::io::Error> {
    thread::Builder::new()
        .name(format!("Thread Nr. {nr}"))
        .spawn(move || -> Result<(), SendError<(u64, bool)>> {
            for num in lower..upper {
                tx.send((num, is_prime(num)))?;
            }
            Ok(())
        })
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
