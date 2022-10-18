// * Find primes starting from 100.000.000 with multi threads
// * - [] check if number is prime
// * - [] define boundaries for each thread
// * - [] create threads
// *    - [] checks if a number is prime
// *    - [] document result
// *    - [] move to next number
// *    - [] publish result

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

    // let upper_boundary = (n as f32).sqrt() as u64;

    // (19..=upper_boundary).step_by(2).all(|num| n % num != 0)
    false
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
}
