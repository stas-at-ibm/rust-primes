// * Find primes starting from 100.000.000 with multi threads
// * - [x] check if number is prime
// *    - [] refactor: use arrays
// *    - [] refactor: extract into module
// * - [x] define boundaries for each thread
// *    - [] refactor: return result instead of panic
// *    - [] refactor: extract into modules
// * - [x] create threads
// *    - [x] checks if a number is prime
// *    - [x] move to next number
// *    - [x] publish result
// * - [] extract into modules
// * - [x] implement unit tests for the boundaries
// * - [] [later] implement unit tests for prime checker - after extraction
// * - [] check out and use concepts from https://doc.rust-lang.org/book/ch12-00-an-io-project.html

use std::{
    sync::mpsc::{self, Receiver, Sender},
    thread::{self, JoinHandle},
};

use colored::Colorize;

fn is_prime(n: u64) -> bool {
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

// highest_number: 100
// threads_amount: 2
// thread_num: 1
// ----------
// step = highest_number / threads_amount    // => 50
// lower_bound = step * (thread_num - 1) + 1 // => 1
// upper_bound = step * thread_num           // => 50
// ----------
// ----------
// highest_number: 100
// threads_amount: 2
// thread_num: 2
// ----------
// step = highest_number / threads_amount    // => 50
// lower_bound = step * (thread_num - 1) + 1 // => 51
// upper_bound = step * thread_num           // => 100
// ----------
// ----------
// highest_number: 101
// threads_amount: 2
// thread_num: 2
// ----------
// step = highest_number / threads_amount    // => 50,5 as u64 => 50
// lower_bound = step * (thread_num - 1) + 1 // => 51
// if threads_amount == thread_num { upper_bound = highest_number }
// else upper_bound = step * thread_num      // => 100
fn boundaries(thread_num: u64, threads_amount: u64, highest_number: u64) -> (u64, u64) {
    if thread_num > threads_amount {
        panic!(
            "Thread number must be smaller than thread amount.\nThread number: {}\nThread amount: {}",
            thread_num,
            threads_amount
        );
    }

    if threads_amount > highest_number {
        panic!(
            "Total number of threads must be smaller than highest number.\nThreads amount: {}\nHighest number: {}.",
            threads_amount,
            highest_number
        );
    }

    let step: u64 = (highest_number / threads_amount) as u64;
    let lower_bound: u64 = step * (thread_num - 1) + 1;

    if threads_amount == thread_num {
        (lower_bound, highest_number)
    } else {
        (lower_bound, step * thread_num)
    }
}

fn range_boundaries(thread_num: u64, threads_amount: u64, search_range: (u64, u64)) -> (u64, u64) {
    let start = search_range.0;
    let end = search_range.1;
    let highest_number = end - start;

    if thread_num > threads_amount {
        panic!(
            "Thread number must be smaller than thread amount.\nThread number: {}\nThread amount: {}",
            thread_num,
            threads_amount
        );
    }

    if threads_amount > highest_number {
        panic!(
            "Total number of threads must be smaller than highest number.\nThreads amount: {}\nHighest number: {}.",
            threads_amount,
            highest_number
        );
    }

    let step: u64 = (highest_number / threads_amount) as u64;
    let lower_bound: u64 = step * (thread_num - 1) + 1;

    if threads_amount == thread_num {
        (start + lower_bound, start + highest_number)
    } else {
        (start + lower_bound, start + (step * thread_num))
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

fn runner(threads_amount: u64, search_range: (u64, u64)) -> Vec<(u64, bool)> {
    let mut handles: Vec<JoinHandle<()>> = vec![];

    let rx: Receiver<(u64, bool)> = {
        let (tx, rx): (Sender<(u64, bool)>, Receiver<(u64, bool)>) = mpsc::channel();

        // todo: break down loop
        for thread_number in 1..=threads_amount {
            let tx_clone: Sender<(u64, bool)> = tx.clone();
            let section: (u64, u64) = range_boundaries(thread_number, threads_amount, search_range);
            println!("Thread {}, Section: {:?}", thread_number, section);

            let handle: JoinHandle<()> = worker(section.0, section.1, tx_clone);
            handles.push(handle);
        }

        rx
    };

    for handle in handles {
        handle.join().unwrap();
    }

    rx.iter().collect()
}

fn print_prime_in_color(checked_numbers: Vec<(u64, bool)>) {
    for num in checked_numbers {
        if num.1 {
            println!("{} is prime.", num.0.to_string().green());
        } else {
            println!("{} is {} prime.", num.0, "not".red());
        }
    }
}

fn main() {
    let threads_amount: u64 = 17;
    let search_range: (u64, u64) = (1_000_000_100_000_u64, 1_000_000_100_200_u64);

    let checked_numbers: Vec<(u64, bool)> = runner(threads_amount, search_range);
    print_prime_in_color(checked_numbers);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn correct_boundaries_for_first_thread_and_even_number() {
        let (lower, upper) = boundaries(1, 2, 100);

        assert_eq!(lower, 1);
        assert_eq!(upper, 50);
    }

    #[test]
    fn correct_boundaries_for_second_thread_and_even_number() {
        let (lower, upper) = boundaries(2, 2, 100);

        assert_eq!(lower, 51);
        assert_eq!(upper, 100);
    }

    #[test]
    fn correct_boundaries_for_first_thread_and_odd_number() {
        let (lower, upper) = boundaries(1, 2, 101);

        assert_eq!(lower, 1);
        assert_eq!(upper, 50);
    }

    #[test]
    fn correct_boundaries_for_second_thread_and_odd_number() {
        let (lower, upper) = boundaries(2, 2, 101);

        assert_eq!(lower, 51);
        assert_eq!(upper, 101);
    }

    #[test]
    fn correct_boundaries_for_third_thread_and_odd_number() {
        let (lower, upper) = boundaries(3, 4, 101);

        assert_eq!(lower, 51);
        assert_eq!(upper, 75);
    }

    #[test]
    fn correct_boundaries_for_third_thread_and_even_number() {
        let (lower, upper) = boundaries(3, 3, 100);

        assert_eq!(lower, 67);
        assert_eq!(upper, 100);
    }

    #[test]
    #[should_panic]
    fn thread_num_is_gt_thread_amount_in_boundaries() {
        boundaries(3, 2, 101);
    }

    #[test]
    #[should_panic]
    fn thread_amount_is_gt_highest_number_in_boundaries() {
        boundaries(2, 2, 1);
    }
}
