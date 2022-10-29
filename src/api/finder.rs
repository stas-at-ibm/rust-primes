use crate::{
    common::thread_pool::ThreadPool,
    model::{
        positive_number::PositiveNumber,
        validation_error::{ValidationError, ValidationErrorKind},
    },
};

use std::{
    ops::Range,
    sync::mpsc::{self, Receiver, Sender},
    thread::{self, JoinHandle},
};

pub fn find_primes_parallel_tx_rx(
    threads_amount: u64,
    lower: u64,
    upper: u64,
) -> Result<Vec<(u64, bool)>, ValidationError> {
    let mut search_range = lower..upper;
    if let Some(err) = validate(threads_amount, &search_range) {
        return Err(err);
    }

    match get_all_boundaries(threads_amount, &mut search_range) {
        Ok(boundaries) => {
            let (rx, handles) = execute_threads_tx_rx(boundaries);

            for handle in handles {
                if let Err(_) = handle.join() {
                    return Err(ValidationError::new(ValidationErrorKind::ThreadPanicError));
                }
            }

            // short form for code below
            return Ok(rx?.iter().collect());
            // match rx {
            //     Ok(rx) => Ok(rx.iter().collect()),
            //     Err(err) => Err(err),
            // }
        }
        Err(err) => Err(err),
    }
}

pub fn find_primes_parallel(
    threads_amount: u64,
    lower: u64,
    upper: u64,
) -> Result<Vec<PositiveNumber>, ValidationError> {
    let mut search_range = lower..upper;
    if let Some(err) = validate(threads_amount, &search_range) {
        return Err(err);
    }

    match get_all_boundaries(threads_amount, &mut search_range) {
        Ok(boundaries) => {
            let handles = execute_threads(boundaries);

            let mut all_checked_numbers: Vec<PositiveNumber> = vec![];
            for handle in handles {
                all_checked_numbers.append(&mut handle.join().unwrap());
            }

            Ok(all_checked_numbers)
        }
        Err(err) => Err(err),
    }
}

pub fn find_primes_parallel_thread_pool(
    threads_amount: u64,
    lower: u64,
    upper: u64,
) -> Result<Vec<PositiveNumber>, ValidationError> {
    let mut search_range = lower..upper;
    if let Some(err) = validate(threads_amount, &search_range) {
        return Err(err);
    }

    match get_all_boundaries(threads_amount, &mut search_range) {
        Ok(boundaries) => {
            let pool = ThreadPool::new(threads_amount as usize);

            for boundary in boundaries {
                pool.execute(|| {
                    check_for_primes(boundary);
                });
            }

            Ok(vec![])
        }
        Err(err) => Err(err),
    }
}

fn check_for_primes(search_range: Range<u64>) {
    let range_size = search_range.end - search_range.start + 1;
    let mut checked_numbers: Vec<PositiveNumber> = Vec::with_capacity(range_size as usize);

    for num in search_range {
        checked_numbers.push(PositiveNumber::new(num, is_prime(num)));
    }

    println!("{:?}", checked_numbers);
}

fn validate(threads_amount: u64, search_range: &Range<u64>) -> Option<ValidationError> {
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

fn get_all_boundaries(
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

fn execute_threads_tx_rx(
    search_ranges_by_thread: Vec<Range<u64>>,
) -> (
    Result<Receiver<(u64, bool)>, ValidationError>,
    Vec<JoinHandle<()>>,
) {
    let (tx, rx): (Sender<(u64, bool)>, Receiver<(u64, bool)>) = mpsc::channel();

    let handles: Vec<JoinHandle<()>> = search_ranges_by_thread
        .iter()
        .map(|search_range| (search_range, tx.clone()))
        .map(|search_range_and_tx| {
            // hack: https://stackoverflow.com/a/62480671/5903780
            let copy = search_range_and_tx.0.start..search_range_and_tx.0.end;
            worker_tx_rx(copy, search_range_and_tx.1)
        })
        .collect();

    (Ok(rx), handles)
}

fn worker_tx_rx(search_range: Range<u64>, tx: Sender<(u64, bool)>) -> JoinHandle<()> {
    thread::spawn(move || {
        for num in search_range {
            tx.send((num, is_prime(num))).unwrap();
        }
    })
}

fn execute_threads(
    search_ranges_by_thread: Vec<Range<u64>>,
) -> Vec<JoinHandle<Vec<PositiveNumber>>> {
    search_ranges_by_thread
        .iter()
        .map(|search_range| {
            // hack: https://stackoverflow.com/a/62480671/5903780
            let copy = search_range.start..search_range.end;
            worker(copy)
        })
        .collect()
}

fn worker(search_range: Range<u64>) -> JoinHandle<Vec<PositiveNumber>> {
    thread::spawn(move || -> Vec<PositiveNumber> {
        let range_size = search_range.end - search_range.start + 1;
        let mut checked_numbers: Vec<PositiveNumber> = Vec::with_capacity(range_size as usize);

        for num in search_range {
            checked_numbers.push(PositiveNumber::new(num, is_prime(num)));
        }

        checked_numbers
    })
}

/*
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
*/

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
