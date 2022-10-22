use colored::Colorize;
use std::{
    sync::mpsc::{self, Receiver, Sender},
    thread::{self, JoinHandle},
};

use self::errors::{ParallelismError, ParallelismErrorKind};

mod errors;

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
    if (n == 1)
        || (n == 2)
        || (n == 3)
        || (n == 5)
        || (n == 7)
        || (n == 11)
        || (n == 13)
        || (n == 17)
        || (n == 19)
    {
        return true;
    }

    if (n % 2 == 0)
        || (n % 3 == 0)
        || (n % 5 == 0)
        || (n % 7 == 0)
        || (n % 11 == 0)
        || (n % 13 == 0)
        || (n % 17 == 0)
        || (n % 19 == 0)
    {
        return false;
    }

    let upper_boundary = (n as f32).sqrt() as u64;

    (19..=upper_boundary).step_by(2).all(|num| n % num != 0)
}

pub fn runner(
    threads_amount: u64,
    search_range: (u64, u64),
) -> Result<Vec<(u64, bool)>, ParallelismError> {
    let mut handles: Vec<JoinHandle<()>> = vec![];

    let rx: Result<Receiver<(u64, bool)>, ParallelismError> = {
        let (tx, rx): (Sender<(u64, bool)>, Receiver<(u64, bool)>) = mpsc::channel();

        for thread_number in 1..=threads_amount {
            let tx_clone: Sender<(u64, bool)> = tx.clone();
            let section: (u64, u64) =
                range_boundaries(thread_number, threads_amount, search_range)?;

            println!("Thread {}, Section: {:?}", thread_number, section);
            let handle: JoinHandle<()> = worker(section.0, section.1, tx_clone);
            handles.push(handle);
        }

        Ok(rx)
    };

    for handle in handles {
        handle.join().unwrap();
    }

    match rx {
        Ok(rx) => Ok(rx.iter().collect()),
        Err(e) => Err(e),
    }
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
