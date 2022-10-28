use api::finder::find_primes_parallel;
use api::printer::print_prime_in_color;
use colored::Colorize;

mod api;
mod model;

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

fn main() {
    // Main will do only
    // * Calling the primes search logic with the argument values
    // * Setting up any other configuration
    // * Calling a run function in lib.rs
    // * Handling the error if run returns an error

    let threads_amount: u64 = 1;
    let lower: u64 = 1_000_000_100_000_u64;
    let upper: u64 = 1_000_000_100_010_u64;
    // let search_range: (u64, u64) = (1_u64, 16_u64);

    println!("================ V1 ===================");
    match find_primes_parallel_tx_rx(threads_amount, lower, upper) {
        Ok(primes) => print_prime_in_color(primes),
        Err(e) => eprintln!("{}: {} ", "error".red(), e),
    };
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
