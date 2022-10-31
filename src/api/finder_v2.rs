use super::common::{break_down_search_range_into_partitions, is_prime, validate};
use crate::model::validation_error::{ValidationError, ValidationErrorKind};

use std::{
    ops::Range,
    sync::mpsc::{self, Receiver, Sender},
    thread::{self, JoinHandle},
};

/// Finds prime numbers using parallelism and channels.
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
) -> Result<Vec<(u64, bool)>, ValidationError> {
    let mut search_range = lower..upper;
    if let Some(err) = validate(threads_amount, &search_range) {
        return Err(err);
    }

    match break_down_search_range_into_partitions(threads_amount, &mut search_range) {
        Ok(boundaries) => {
            let (rx, handles) = execute_threads_tx_rx(boundaries);

            for handle in handles {
                if let Err(_) = handle.join() {
                    return Err(ValidationError::new(ValidationErrorKind::ThreadPanicError));
                }
            }

            // Ok(rx?.iter().collect()); is the short form for
            // match rx {
            //     Ok(rx) => Ok(rx.iter().collect()),
            //     Err(err) => Err(err),
            // }
            return Ok(rx?.iter().collect());
        }
        Err(err) => Err(err),
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
