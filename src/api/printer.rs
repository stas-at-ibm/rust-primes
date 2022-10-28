use crate::model::positive_number::PositiveNumber;
use colored::Colorize;

pub fn print_prime_in_color(list_with_primes: Vec<(u64, bool)>) {
    for num in list_with_primes {
        if num.1 {
            println!("{} is prime.", num.0.to_string().green());
        } else {
            println!("{} is {} prime.", num.0, "not".red());
        }
    }
}

pub fn print_positive_number_prime_in_color(list_with_primes: Vec<PositiveNumber>) {
    for num in list_with_primes {
        if num.is_prime() {
            println!("{} is prime.", num.value().to_string().green());
        } else {
            println!("{} is {} prime.", num.value(), "not".red());
        }
    }
}
