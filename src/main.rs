// * Find primes starting from 100.000.000 with multi threads
// * - [x] check if number is prime
// *    - [] refactor: use arrays
// *    - [] refactor: extract into module
// * - [x] define boundaries for each thread
// *    - [x] refactor: return result instead of panic
// *    - [] refactor: extract into modules
// * - [x] create threads
// *    - [x] checks if a number is prime
// *    - [x] move to next number
// *    - [x] publish result
// * - [x] extract into modules
// * - [ ] use lib and bin modules, put primes and errors into lib
// * - [x] implement unit tests for the boundaries
// * - [] [later] implement unit tests for prime checker - after extraction
// * - [] check out and use concepts from https://doc.rust-lang.org/book/ch12-00-an-io-project.html
// * - [] range_boundaries: bug if search range starts with 1
// * - [] range_boundaries: unit test
// * - [] runner: break down loop
// * - [x] [useless] thread: pass in function, own module, tx optional, generics for tx

mod primes;
use primes::{print_prime_in_color, runner};

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
