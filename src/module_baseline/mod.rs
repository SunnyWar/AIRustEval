use crate::common::AICodeGenStatus;
use crate::common::CandidateInfo;
use crate::common::CandidateInfo2;
use chrono::NaiveDate;
use std::vec;

// Note: this functions was hand-crafted and cannot change.
// They represent the baseline that each AI has to improve upon.

// add to the vectors as more attempts an this function are made by the AI
pub fn get_candidates() -> CandidateInfo {
    CandidateInfo::new(
        String::from("Baseline"),
        vec!["levenshstein distance".to_string()],
        vec![NaiveDate::from_ymd_opt(2025, 1, 2).unwrap()],
        vec![AICodeGenStatus::Ok],
        vec![levenshtein_distance],
    )
}

pub fn get_candidates2() -> CandidateInfo2 {
    CandidateInfo2::new(
        String::from("Baseline"),
        vec![
            "sum of divisors".to_string(),
            "count of primes".to_string(),
            "fibonacci".to_string(),
            "highly composite".to_string(),
            "sum of proper divisors".to_string(),
        ],
        vec![
            NaiveDate::from_ymd_opt(2025, 2, 3).unwrap(),
            NaiveDate::from_ymd_opt(2025, 2, 3).unwrap(),
            NaiveDate::from_ymd_opt(2025, 2, 3).unwrap(),
            NaiveDate::from_ymd_opt(2025, 2, 3).unwrap(),
            NaiveDate::from_ymd_opt(2025, 2, 3).unwrap(),
        ],
        vec![
            AICodeGenStatus::Ok,
            AICodeGenStatus::Ok,
            AICodeGenStatus::Ok,
            AICodeGenStatus::Ok,
            AICodeGenStatus::Ok,
        ],
        vec![
            sum_of_divisors,
            count_primes,
            fibonacci,
            highly_composite,
            sum_of_proper_divisors,
        ],
    )
}

#[inline(never)]
#[allow(clippy::needless_range_loop)]
// You are a highly skilled Rust developer. Your task is to optimize the following Rust function for maximum speed and efficiency. The code will run on a x86-x64 system with an AMD Ryzen 7 processor. Readability of the code is not important. Please provide the optimized code. Here is the function that needs optimization:
pub fn levenshtein_distance(s: &str, t: &str) -> usize {
    let m = s.len();
    let n = t.len();

    // Create a 2D matrix to store the distances
    let mut matrix = vec![vec![0; n + 1]; m + 1];

    // Initialize the first row and column of the matrix
    for i in 0..=m {
        matrix[i][0] = i;
    }
    for j in 0..=n {
        matrix[0][j] = j;
    }

    // Compute the Levenshtein distance
    for i in 1..=m {
        for j in 1..=n {
            let cost = if s.chars().nth(i - 1) == t.chars().nth(j - 1) {
                0 // No cost if characters are the same
            } else {
                1 // Cost of 1 if characters are different
            };

            matrix[i][j] = *[
                matrix[i - 1][j] + 1,        // Deletion
                matrix[i][j - 1] + 1,        // Insertion
                matrix[i - 1][j - 1] + cost, // Substitution
            ]
            .iter()
            .min()
            .unwrap();
        }
    }

    // The last element of the matrix is the Levenshtein distance
    matrix[m][n]
}

/// Returns the sum of all divisors of the given number `n`.
/// Naive implementation iterates up to `n`.
#[inline(never)]
// You are a highly skilled Rust developer. Your task is to optimize the following Rust function for maximum speed and efficiency. The code will run on a x86-x64 system with an AMD Ryzen 7 processor. Readability of the code is not important. Please provide the optimized code. Here is the function that needs optimization:
pub fn sum_of_divisors(n: u64) -> u64 {
    (1..=n).filter(|&i| n % i == 0).sum()
}

/// Counts the number of prime numbers less than `n`.
/// Naive implementation checks every number for primality.
#[inline(never)]
// You are a highly skilled Rust developer. Your task is to optimize the following Rust function for maximum speed and efficiency. The code will run on a x86-x64 system with an AMD Ryzen 7 processor. Readability of the code is not important. Please provide the optimized code. Here is the function that needs optimization:
pub fn count_primes(n: u64) -> u64 {
    (2..n).filter(|&x| is_prime(x)).count() as u64
}

/// Helper function to check if a number is prime.
fn is_prime(num: u64) -> bool {
    if num < 2 {
        return false;
    }
    for i in 2..=((num as f64).sqrt() as u64) {
        if num % i == 0 {
            return false;
        }
    }
    true
}

/// Computes the nth Fibonacci number.
/// Naive implementation uses recursion.
#[inline(never)]
// You are a highly skilled Rust developer. Your task is to optimize the following Rust function for maximum speed and efficiency. The code will run on a x86-x64 system with an AMD Ryzen 7 processor. Readability of the code is not important. Please provide the optimized code. Here is the function that needs optimization:
pub fn fibonacci(n: u64) -> u64 {
    match n {
        0 => 0,
        1 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

/// Finds the first highly composite number greater than `n`.
/// A naive implementation checks every number up to n, counting divisors for each.
#[inline(never)]
// You are a highly skilled Rust developer. Your task is to optimize the following Rust function for maximum speed and efficiency. The code will run on a x86-x64 system with an AMD Ryzen 7 processor. Readability of the code is not important. Please provide the optimized code. Here is the function that needs optimization:
pub fn highly_composite(n: u64) -> u64 {
    (n + 1..)
        .find(|&x| count_divisors(x) > count_divisors(n))
        .unwrap()
}

/// Helper function to count the number of divisors.
fn count_divisors(n: u64) -> u64 {
    (1..=n).filter(|&i| n % i == 0).count() as u64
}

/// Helper function to calculate the sum of proper divisors.
#[inline(never)]
// You are a highly skilled Rust developer. Your task is to optimize the following Rust function for maximum speed and efficiency. The code will run on a x86-x64 system with an AMD Ryzen 7 processor. Readability of the code is not important. Please provide the optimized code. Here is the function that needs optimization:
pub fn sum_of_proper_divisors(n: u64) -> u64 {
    (1..n).filter(|&i| n % i == 0).sum()
}
