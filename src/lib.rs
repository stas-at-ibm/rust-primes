mod errors;
use colored::Colorize;
use errors::{ValidationError, ValidationErrorKind};
use std::{
    sync::mpsc::{self, Receiver, Sender},
    thread::{self, JoinHandle},
};

pub fn print_prime_in_color(list_with_primes: Vec<(u64, bool)>) {
    for num in list_with_primes {
        if num.1 {
            println!("{} is prime.", num.0.to_string().green());
        } else {
            println!("{} is {} prime.", num.0, "not".red());
        }
    }
}

pub fn find_primes_parallel(
    threads_amount: u64,
    search_range: (u64, u64),
) -> Result<Vec<(u64, bool)>, ValidationError> {
    if threads_amount == 0 {
        return Err(ValidationError::new(ValidationErrorKind::ZeroThreadsError));
    }

    let (rx, handles) = start_threads_no_unwrap(threads_amount, search_range);

    for handle in handles {
        if let Err(_) = handle.join() {
            return Err(ValidationError::new(ValidationErrorKind::ThreadPanicError));
        }
    }

    match rx {
        Ok(rx) => Ok(rx.iter().collect()),
        Err(e) => Err(e),
    }
}

pub fn XXX_find_primes_parallel(
    threads_amount: u64,
    search_range: (u64, u64),
) -> Result<Vec<(u64, bool)>, ValidationError> {
    // validation
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

    let (rx, handles) = start_threads_no_unwrap(threads_amount, search_range);

    for handle in handles {
        if let Err(_) = handle.join() {
            return Err(ValidationError::new(ValidationErrorKind::ThreadPanicError));
        }
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
    Result<Receiver<(u64, bool)>, ValidationError>,
    Vec<JoinHandle<()>>,
) {
    let handles: Vec<JoinHandle<()>>;

    // hack: encapsulated into "{}" so that the receiver closes when the threads finish
    let rx: Result<Receiver<(u64, bool)>, ValidationError> = {
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

fn start_threads_no_unwrap(
    threads_amount: u64,
    search_range: (u64, u64),
) -> (
    Result<Receiver<(u64, bool)>, ValidationError>,
    Vec<JoinHandle<()>>,
) {
    let (tx, rx): (Sender<(u64, bool)>, Receiver<(u64, bool)>) = mpsc::channel();

    let handles: Vec<JoinHandle<()>> = (1..=threads_amount)
        .map(|_| tx.clone())
        .enumerate()
        .map(|(thread_number, tx)| {
            (
                // move this out to client
                range_boundaries(thread_number as u64 + 1, threads_amount, search_range).unwrap(),
                tx,
            )
        })
        .map(|range_and_tx| worker(range_and_tx.0 .0, range_and_tx.0 .1, range_and_tx.1))
        .collect();

    // let mut txs: Vec<Sender<(u64, bool)>> = (1..=threads_amount).map(|_| tx.clone()).collect();
    // let mut boundaries: Vec<Result<(u64, u64), ParallelismError>> = (1..=threads_amount)
    //     .map(|thread_number| range_boundaries(thread_number, threads_amount, search_range))
    //     .collect();

    // let boundaries_error;
    // for (i, boundary) in boundaries.iter().enumerate() {
    //     match boundary {
    //         Ok(value) => {
    //             let handle = worker(value.0, value.1, txs.remove(i));
    //             handles.push(handle);
    //         }
    //         Err(e) => {
    //             boundaries_error = e;
    //             break;
    //         }
    //     };
    // }

    (Ok(rx), handles)
}

// let mut txs: Vec<Sender<(u64, bool)>> = (1..=threads_amount).map(|_| tx.clone()).collect();

// let mut boundaries: Vec<Result<(u64, u64), ParallelismError>> = (1..=threads_amount)
//     .map(|thread_number| range_boundaries(thread_number, threads_amount, search_range))
//     .collect();

// let boundaries_error;
// for (i, boundary) in boundaries.iter().enumerate() {
//     match boundary {
//         Ok(value) => {
//             let handle = worker(value.0, value.1, txs.remove(i));
//             handles.push(handle);
//         }
//         Err(e) => {
//             boundaries_error = e;
//             break;
//         }
//     };
// }

fn range_boundaries(
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
