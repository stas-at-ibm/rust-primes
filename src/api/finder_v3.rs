use crate::{
    infrastructure::thread_pool::ThreadPool,
    model::{
        positive_number::PositiveNumber, search_range::SearchRange,
        validation_error::ValidationError,
    },
};

use std::{
    ops::Range,
    sync::mpsc::{self, Receiver},
};

use super::common::is_prime;

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
    threads: u64,
    lower: u64,
    upper: u64,
) -> Result<Vec<PositiveNumber>, ValidationError> {
    let search_range = SearchRange::new(lower, upper, threads)?;

    let pool = ThreadPool::new(threads as usize);
    let rx: Result<Receiver<Vec<PositiveNumber>>, ValidationError> = {
        let (tx, rx) = mpsc::channel();

        for partition in search_range.partitions() {
            let tx_copy = tx.clone();
            // needed to prevent dangling reference error because partition moves into an
            // async function
            let partition_copy = partition.clone();
            pool.execute(move || {
                let checked_nums = check_for_primes(partition_copy);
                tx_copy.send(checked_nums).unwrap();
            });
        }

        Ok(rx)
    };

    let mut all_checked_nums: Vec<PositiveNumber> = vec![];
    for mut checked_partition in rx?.iter() {
        all_checked_nums.append(&mut checked_partition);
    }

    Ok(all_checked_nums)
}

fn check_for_primes(search_range: Range<u64>) -> Vec<PositiveNumber> {
    let range_size = search_range.end - search_range.start + 1;
    let mut checked_numbers: Vec<PositiveNumber> = Vec::with_capacity(range_size as usize);

    for num in search_range {
        checked_numbers.push(PositiveNumber::new(num, is_prime(num)));
    }

    checked_numbers
}
