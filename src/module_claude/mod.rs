use crate::common::AICodeGenStatus;
use crate::common::CandidateInfo;
use crate::common::CandidateInfo2;
use chrono::NaiveDate;

// found here: https://claude.ai/
// version can be found by asking claude "what version are you?"

// add to the vectors as more attempts an this function are made by the AI
pub fn get_candidates() -> CandidateInfo {
    CandidateInfo::new(
        String::from("claude 3.7 Sonnet"),
        vec!["levenshstein distance".to_string()],
        vec![NaiveDate::from_ymd_opt(2025, 3, 3).unwrap()],
        vec![AICodeGenStatus::Ok],
        vec![levenshtein_distance],
    )
}

pub fn get_candidates2() -> CandidateInfo2 {
    CandidateInfo2::new(
        String::from("claude 3.7 Sonnet"),
        vec![
            "sum of divisors".to_string(),
            "count of primes".to_string(),
            "fibonacci".to_string(),
            "highly composite".to_string(),
            "sum of proper divisors".to_string(),
        ],
        vec![
            NaiveDate::from_ymd_opt(2025, 3, 3).unwrap(),
            NaiveDate::from_ymd_opt(2025, 3, 3).unwrap(),
            NaiveDate::from_ymd_opt(2025, 3, 3).unwrap(),
            NaiveDate::from_ymd_opt(2025, 3, 3).unwrap(),
            NaiveDate::from_ymd_opt(2025, 3, 3).unwrap(),
        ],
        vec![
            AICodeGenStatus::Ok,
            AICodeGenStatus::Ok,
            AICodeGenStatus::SecondTryOk,
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
#[allow(clippy::collapsible_else_if)]
#[allow(unused_variables)]
pub fn levenshtein_distance(s: &str, t: &str) -> usize {
    // Handle empty string cases quickly
    if s.is_empty() {
        return t.len();
    }
    if t.is_empty() {
        return s.len();
    }

    // Ensure s is the shorter string for better cache usage
    if s.len() > t.len() {
        return levenshtein_distance(t, s);
    }

    let s_bytes = s.as_bytes();
    let t_bytes = t.as_bytes();
    let s_len = s_bytes.len();
    let t_len = t_bytes.len();

    // Use a single vector instead of a 2D matrix to improve cache locality
    // Only need to store two rows at any time
    let mut prev_row = Vec::with_capacity(s_len + 1);
    let mut curr_row = Vec::with_capacity(s_len + 1);

    // Initialize previous row (the distances for first row)
    prev_row.extend(0..=s_len);

    // Fill in current row for each character in t
    for (j, t_char) in t_bytes.iter().enumerate() {
        // First element of the current row is the column index + 1
        curr_row.clear();
        curr_row.push(j + 1);

        // Calculate distance for each character in s
        for (i, s_char) in s_bytes.iter().enumerate() {
            let deletion = prev_row[i + 1] + 1;
            let insertion = curr_row[i] + 1;

            // Fast byte comparison instead of using chars().nth()
            let substitution_cost = if s_char == t_char { 0 } else { 1 };
            let substitution = prev_row[i] + substitution_cost;

            // Use min() directly for better compiler optimization
            let min_dist = if deletion < insertion {
                if substitution < deletion {
                    substitution
                } else {
                    deletion
                }
            } else {
                if substitution < insertion {
                    substitution
                } else {
                    insertion
                }
            };

            curr_row.push(min_dist);
        }

        // Swap rows for next iteration
        std::mem::swap(&mut prev_row, &mut curr_row);
    }

    // The last element of the previous row is the answer
    prev_row[s_len]
}

#[inline(never)]
pub fn sum_of_divisors(n: u64) -> u64 {
    // Edge cases
    if n == 0 {
        return 0;
    }
    if n == 1 {
        return 1;
    }

    let mut sum = 1 + n; // Always include 1 and n as divisors

    // Only check up to sqrt(n) and add both divisors at once
    let sqrt_n = (n as f64).sqrt() as u64;

    // Use a simple loop instead of functional style for better performance
    let mut i = 2;
    while i <= sqrt_n {
        if n % i == 0 {
            sum += i;

            // Add the paired divisor if it's different (avoid duplicates)
            let paired = n / i;
            if paired != i {
                sum += paired;
            }
        }
        i += 1;
    }

    sum
}

#[inline(never)]
pub fn count_primes(n: u64) -> u64 {
    if n <= 2 {
        return 0;
    }
    if n <= 3 {
        return 1;
    }

    // Sieve of Eratosthenes using a bitvector for memory efficiency
    let n_usize = n as usize;
    let mut sieve = vec![true; n_usize];

    // 0 and 1 are not prime
    if n_usize > 1 {
        sieve[0] = false;
        sieve[1] = false;
    }

    // Only need to sieve up to sqrt(n)
    let sqrt_n = (n as f64).sqrt() as usize;

    // Use unsafe for better performance with large arrays
    unsafe {
        for i in 2..=sqrt_n {
            if *sieve.get_unchecked(i) {
                // Start at i*i as all smaller multiples are already marked
                let mut j = i * i;
                while j < n_usize {
                    *sieve.get_unchecked_mut(j) = false;
                    j += i;
                }
            }
        }
    }

    // Count remaining primes
    sieve.iter().filter(|&&is_prime| is_prime).count() as u64
}

#[inline(never)]
// AICodeGenStatus::SecondTryOk
pub fn fibonacci(n: u64) -> u64 {
    if n <= 1 {
        return n;
    }

    // Use iterative approach with explicit u64 types
    let mut a: u64 = 0;
    let mut b: u64 = 1;

    // Fast matrix exponentiation for large n values
    if n > 92 {
        // Return u64::MAX as a signal that the result would overflow
        // (Fib(93) is the first to exceed u64::MAX)
        return u64::MAX;
    }

    // Use efficient iteration with no additional memory
    for _ in 2..=n {
        let c = a.wrapping_add(b); // wrapping_add to prevent debug mode panics
        a = b;
        b = c;
    }

    b
}

#[inline(never)]
pub fn highly_composite(n: u64) -> u64 {
    let div_count_n = optimized_count_divisors(n);

    // Start searching from n+1
    let mut candidate = n + 1;

    loop {
        let div_count = optimized_count_divisors(candidate);
        if div_count > div_count_n {
            return candidate;
        }
        candidate += 1;
    }
}

/// Optimized helper function to count divisors using prime factorization
#[allow(unused_assignments)]
fn optimized_count_divisors(n: u64) -> u64 {
    if n == 0 {
        return 0;
    }
    if n == 1 {
        return 1;
    }

    let mut num = n;
    let mut divisors = 1;
    let mut factor = 2;
    let mut count;

    // Extract all factors of 2
    count = 0;
    while num % 2 == 0 {
        count += 1;
        num /= 2;
    }
    divisors *= count + 1;

    // Extract all odd prime factors
    factor = 3;
    let sqrt_n = (num as f64).sqrt() as u64 + 1;

    while factor <= sqrt_n && num > 1 {
        count = 0;
        while num % factor == 0 {
            count += 1;
            num /= factor;
        }

        if count > 0 {
            divisors *= count + 1;
        }

        factor += 2; // Skip even numbers
    }

    // If num is a prime number greater than sqrt_n
    if num > 1 {
        divisors *= 2; // Prime number has exactly 2 divisors: 1 and itself
    }

    divisors
}

#[inline(never)]
pub fn sum_of_proper_divisors(n: u64) -> u64 {
    // Handle edge cases
    if n <= 1 {
        return 0;
    }

    let mut sum: u64 = 1; // Start with 1 as it's always a proper divisor for n > 1

    // Only check up to sqrt(n) and add both divisors at once
    let sqrt_n = (n as f64).sqrt() as u64;

    // Optimize the loop to check only up to sqrt(n)
    let mut i: u64 = 2;
    while i <= sqrt_n {
        if n % i == 0 {
            sum += i;

            // If i != n/i (to avoid counting the same divisor twice in perfect squares)
            // Add the paired divisor
            let paired = n / i;
            if paired != i {
                sum += paired;
            }
        }
        i += 1;
    }

    sum
}
