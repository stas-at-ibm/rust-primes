use crate::model::{positive_number::PositiveNumber, validation_error::ValidationError};

use std::{
    ops::Range,
    thread::{self, JoinHandle},
};

use super::common::{break_down_search_range_into_partitions, is_prime, validate};

/// Finds prime numbers using a parallelism.
///
/// The threads_amount is the number of threads which will be used.
///
/// lower and upper define the range in which to search for primes.
///
/// # Panics
///
/// The `handle.join` fails.
pub fn find_primes_parallel(
    threads_amount: u64,
    lower: u64,
    upper: u64,
) -> Result<Vec<PositiveNumber>, ValidationError> {
    let mut search_range = lower..upper;
    if let Some(err) = validate(threads_amount, &search_range) {
        return Err(err);
    }

    match break_down_search_range_into_partitions(threads_amount, &mut search_range) {
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
