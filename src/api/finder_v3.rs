use crate::{
    common::thread_pool::ThreadPool,
    model::{positive_number::PositiveNumber, validation_error::ValidationError},
};

use std::{
    ops::Range,
    sync::mpsc::{self, Receiver},
};

use super::common::{get_all_boundaries, is_prime, validate};

/// Finds prime numbers using a thread pool and channels.
///
/// The threads_amount is the number of threads which will be used.
///
/// lower and upper define the range in which to search for primes.
///
/// # Panics
///
/// The `tx.send` function will panic if the receiver is closed beforehand.
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
            let pool = ThreadPool::new(threads_amount as usize);
            let rx: Result<Receiver<Vec<PositiveNumber>>, ValidationError> = {
                let (tx, rx) = mpsc::channel();

                for boundary in boundaries {
                    let tx_copy = tx.clone();
                    pool.execute(move || {
                        let checked_nums = check_for_primes(boundary);
                        tx_copy.send(checked_nums).unwrap();
                    });
                }

                Ok(rx)
            };

            let mut all_checked_nums: Vec<PositiveNumber> = vec![];
            for mut checked_section in rx?.iter() {
                all_checked_nums.append(&mut checked_section);
            }

            Ok(all_checked_nums)
        }
        Err(err) => Err(err),
    }
}

fn check_for_primes(search_range: Range<u64>) -> Vec<PositiveNumber> {
    let range_size = search_range.end - search_range.start + 1;
    let mut checked_numbers: Vec<PositiveNumber> = Vec::with_capacity(range_size as usize);

    for num in search_range {
        checked_numbers.push(PositiveNumber::new(num, is_prime(num)));
    }

    checked_numbers
}
