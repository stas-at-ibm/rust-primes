mod errors;
use colored::Colorize;
use errors::{ParallelismError, ParallelismErrorKind};
use std::{
    sync::mpsc::{self, Receiver, Sender},
    thread::{self, JoinHandle},
};

pub fn print_prime_in_color(checked_numbers: Vec<(u64, bool)>) {
    for num in checked_numbers {
        if num.1 {
            println!("{} is prime.", num.0.to_string().green());
        } else {
            println!("{} is {} prime.", num.0, "not".red());
        }
    }
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

pub fn find_primes_parallel(
    threads_amount: u64,
    search_range: (u64, u64),
) -> Result<Vec<(u64, bool)>, ParallelismError> {
    let (rx, handles) = start_threads(threads_amount, search_range);

    for handle in handles {
        handle.join().unwrap();
    }

    match rx {
        Ok(rx) => Ok(rx.iter().collect()),
        Err(e) => Err(e),
    }
}

fn start_threads(
    threads_amount: u64,
    search_range: (u64, u64),
) -> (
    Result<Receiver<(u64, bool)>, ParallelismError>,
    Vec<JoinHandle<()>>,
) {
    let handles: Vec<JoinHandle<()>>;

    // hack: encapsulated into "{}" so that the receiver closes when the threads finish
    let rx: Result<Receiver<(u64, bool)>, ParallelismError> = {
        let (tx, rx): (Sender<(u64, bool)>, Receiver<(u64, bool)>) = mpsc::channel();

        handles = (1..=threads_amount)
            .map(|_| tx.clone())
            .enumerate()
            .map(|(thread_number, tx)| {
                (
                    range_boundaries(thread_number as u64 + 1, threads_amount, search_range)
                        .unwrap(),
                    tx,
                )
            })
            .map(|range_and_tx| worker(range_and_tx.0 .0, range_and_tx.0 .1, range_and_tx.1))
            .collect();

        Ok(rx)
    };

    (rx, handles)
}

fn range_boundaries(
    thread_num: u64,
    threads_amount: u64,
    search_range: (u64, u64),
) -> Result<(u64, u64), ParallelismError> {
    let start = search_range.0;
    let end = search_range.1;
    let highest_number = end - start;

    if thread_num > threads_amount {
        return Err(ParallelismError::new(
            ParallelismErrorKind::ThreadNumberError,
        ));
    }

    if threads_amount > highest_number {
        return Err(ParallelismError::new(
            ParallelismErrorKind::ThreadAmountError,
        ));
    }

    let step: u64 = (highest_number / threads_amount) as u64;
    let lower_bound: u64 = step * (thread_num - 1) + 1;

    if threads_amount == thread_num {
        Ok((start + lower_bound, start + highest_number))
    } else {
        Ok((start + lower_bound, start + (step * thread_num)))
    }
}
