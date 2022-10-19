// * Find primes starting from 100.000.000 with multi threads
// * - [x] check if number is prime
// *    - [] refactor
// * - [] define boundaries for each thread
// * - [] create threads
// *    - [] checks if a number is prime
// *    - [] document result
// *    - [] move to next number
// *    - [] publish result
// * - [] extract into modules
// * - [] implement unit tests

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
    let step: u64 = (highest_number / threads_amount) as u64;
    let lower_bound: u64 = step * (thread_num - 1) + 1;

    if threads_amount == thread_num {
        (lower_bound, highest_number)
    } else {
        (lower_bound, step * thread_num)
    }
}

fn main() {
    for n in 1..50 {
        let colored_prime;
        let it_is_prime = is_prime(n);
        if it_is_prime {
            colored_prime = it_is_prime.to_string().green();
        } else {
            colored_prime = it_is_prime.to_string().red();
        }

        println!("Number: {} is prime: {}", n, colored_prime);
    }

    println!("Thread number: 1");
    println!("Thread amount: 2");
    println!("Highest number: 100");
    println!("Upper bound: {}", boundaries(2, 2, 100).0);
    println!("Lower bound: {}", boundaries(2, 2, 100).1);
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
