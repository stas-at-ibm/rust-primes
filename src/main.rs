use api::finder::{find_primes_parallel, find_primes_parallel_tx_rx};
use api::printer::{print_positive_number_prime_in_color, print_prime_in_color};
use colored::Colorize;

use crate::api::finder::find_primes_parallel_thread_pool;

mod api;
mod common;
mod model;

fn main() {
    let threads_amount: u64 = 2;
    let lower: u64 = 1_000_000_100_000_u64;
    let upper: u64 = 1_000_000_100_010_u64;
    // let search_range: (u64, u64) = (1_u64, 16_u64);

    println!("================ V1 ===================");
    match find_primes_parallel_tx_rx(threads_amount, lower, upper) {
        Ok(primes) => print_prime_in_color(primes),
        Err(e) => eprintln!("{}: {} ", "error".red(), e),
    };

    println!("================ V2 ===================");
    match find_primes_parallel(threads_amount, lower, upper) {
        Ok(primes) => print_positive_number_prime_in_color(primes),
        Err(e) => eprintln!("{}: {} ", "error".red(), e),
    };

    println!("================ V3 ===================");
    match find_primes_parallel_thread_pool(threads_amount, lower, upper) {
        Ok(primes) => print_positive_number_prime_in_color(primes),
        Err(e) => eprintln!("{}: {} ", "error".red(), e),
    };
}
