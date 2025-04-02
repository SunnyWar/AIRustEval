use chrono::NaiveDate;

use crate::common::AICodeGenStatus;
use crate::common::CandidateInfo;
use crate::common::CandidateInfo2;

// found here: https://gemini.google.com/
// version is listed on page and can the selected

// add to the vectors as more attempts an this function are made by the AI
pub fn get_candidates() -> CandidateInfo {
    CandidateInfo::new(
        String::from("gemini 2.0 Flash"),
        vec!["levenshstein distance".to_string()],
        vec![NaiveDate::from_ymd_opt(2025, 1, 2).unwrap()],
        vec![AICodeGenStatus::Ok],
        vec![levenshtein_distance],
    )
}

pub fn get_candidates2() -> CandidateInfo2 {
    CandidateInfo2::new(
        String::from("gemini 2.5 Pro"),
        vec![
            "sum of divisors".to_string(),
        ],
        vec![
            NaiveDate::from_ymd_opt(2025, 4, 1).unwrap(),

        ],
        vec![
            AICodeGenStatus::Ok,
        ],
        vec![
            sum_of_divisors,

        ],
    )
}

//"sum of divisors".to_string(),
//"count of primes".to_string(),
//"fibonacci".to_string(),
//"highly composite".to_string(),
//"sum of proper divisors".to_string(),

#[inline(never)]
#[allow(clippy::needless_range_loop)]
pub fn levenshtein_distance(s: &str, t: &str) -> usize {
    if s.is_empty() {
        return t.len();
    }
    if t.is_empty() {
        return s.len();
    }

    let s: Vec<u8> = s.bytes().collect();
    let t: Vec<u8> = t.bytes().collect();

    let m = s.len();
    let n = t.len();

    let mut v0: Vec<usize> = (0..=n).collect();
    let mut v1: Vec<usize> = vec![0; n + 1];

    for i in 0..m {
        v1[0] = i + 1;

        for j in 0..n {
            let cost = if s[i] == t[j] { 0 } else { 1 };
            v1[j + 1] = (v1[j] + 1).min(v0[j + 1] + 1).min(v0[j] + cost);
        }

        std::mem::swap(&mut v0, &mut v1);
    }

    v0[n]
}

#[inline(never)]
pub fn sum_of_divisors(n: u64) -> u64 {
    // Handle edge cases explicitly for potential minor speedup
    // and correctness. Sum of divisors of 0 is often considered 0.
    if n == 0 {
        return 0;
    }
    // Sum of divisors of 1 is 1.
    if n == 1 {
        return 1;
    }

    let mut sum: u64 = 0;

    // Calculate the integer square root.
    // Using f64::sqrt is generally fast on x86-64.
    // Note: For extremely large u64 values near the limit, f64 precision
    // might become an issue, but it's highly unlikely for typical inputs.
    // An integer-only sqrt algorithm could be used if this becomes a bottleneck
    // or precision issue in very specific edge cases, but adds complexity.
    let sqrt_n = (n as f64).sqrt() as u64;

    // Iterate from 1 up to (and including) the square root of n.
    // Using a simple loop often allows the compiler to optimize better
    // than iterator chains for tight numerical code.
    for i in 1..=sqrt_n {
        // Check if i is a divisor
        if n % i == 0 {
            // i is a divisor. Calculate its pair.
            let pair = n / i;

            // If i * i == n, then i is the square root, and i == pair.
            // Add it only once to avoid double counting.
            if i == pair {
                // Using checked_add for safety against potential overflow,
                // though for sum-of-divisors, overflow is less common unless n is huge.
                // If *maximum* speed is needed and overflow can be ruled out or ignored,
                // plain `sum += i` could be used, but checked_add is safer with
                // minimal performance cost on modern CPUs. Let's assume safety is still desired.
                sum = sum.checked_add(i).expect("Overflow occurred");
            } else {
                // i and pair are distinct divisors. Add both.
                sum = sum.checked_add(i).expect("Overflow occurred");
                sum = sum.checked_add(pair).expect("Overflow occurred");
            }
        }
    }

    sum
}