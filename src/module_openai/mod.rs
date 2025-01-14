use chrono::NaiveDate;

use crate::common::AICodeGenStatus;
use crate::common::CandidateInfo;

// found here: https://chatgpt.com/
// version can be found prompting: "what version are you?"

// add to the vectors as more attempts an this function are made by the AI
pub fn get_candidates() -> CandidateInfo {
    CandidateInfo::new(
        String::from("ChatGPT, version 2"),
        vec!["levenshstein distance".to_string()],
        vec![NaiveDate::from_ymd_opt(2025, 1, 2).unwrap()],
        vec![AICodeGenStatus::Ok],
        vec![levenshtein_distance],
    )
}

#[inline(never)]
pub fn levenshtein_distance(s: &str, t: &str) -> usize {
    let m = s.len();
    let n = t.len();

    if m == 0 {
        return n;
    }
    if n == 0 {
        return m;
    }

    // Convert strings to byte slices for faster character access
    let s_bytes = s.as_bytes();
    let t_bytes = t.as_bytes();

    // Use two rows for computation
    let mut prev_row: Vec<usize> = (0..=n).collect();
    let mut curr_row: Vec<usize> = vec![0; n + 1];

    for i in 1..=m {
        curr_row[0] = i;
        for j in 1..=n {
            let cost = if s_bytes[i - 1] == t_bytes[j - 1] {
                0
            } else {
                1
            };

            curr_row[j] = *[
                prev_row[j] + 1,        // Deletion
                curr_row[j - 1] + 1,    // Insertion
                prev_row[j - 1] + cost, // Substitution
            ]
            .iter()
            .min()
            .unwrap();
        }
        std::mem::swap(&mut prev_row, &mut curr_row);
    }

    prev_row[n]
}

#[inline(never)]
pub fn sum_of_divisors(n: u64) -> u64 {
    let mut sum = 0;
    let sqrt_n = (n as f64).sqrt() as u64;

    for i in 1..=sqrt_n {
        if n % i == 0 {
            sum += i; // Add the divisor
            if i != n / i {
                sum += n / i; // Add the complementary divisor
            }
        }
    }

    sum
}
