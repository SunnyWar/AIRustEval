use chrono::NaiveDate;

use crate::common::AICodeGenStatus;
use crate::common::CandidateInfo;

// found here: https://syntha.ai/optimizers/rust
// I couldn't find a way to get the name or version of the AI engine used

// add to the vectors as more attempts an this function are made by the AI
pub fn get_candidates() -> CandidateInfo {
    CandidateInfo::new(
        String::from("syntha.ai"),
        vec!["levenshstein distance".to_string()],
        vec![NaiveDate::from_ymd_opt(2025, 1, 2).unwrap()],
        vec![AICodeGenStatus::Ok],
        vec![levenshtein_distance],
    )
}

//"sum of divisors".to_string(),
//"count of primes".to_string(),
//"fibonacci".to_string(),
//"highly composite".to_string(),
//"sum of proper divisors".to_string(),

#[inline(never)]
pub fn levenshtein_distance(s: &str, t: &str) -> usize {
    let (m, n) = (s.len(), t.len());
    let mut dp = vec![vec![0; n + 1]; m + 1];

    for i in 0..=m {
        for j in 0..=n {
            if i == 0 {
                dp[i][j] = j;
            } else if j == 0 {
                dp[i][j] = i;
            } else {
                let cost = if s.chars().nth(i - 1) == t.chars().nth(j - 1) {
                    0
                } else {
                    1
                };
                dp[i][j] = *[
                    dp[i - 1][j] + 1,        // Deletion
                    dp[i][j - 1] + 1,        // Insertion
                    dp[i - 1][j - 1] + cost, // Substitution
                ]
                .iter()
                .min()
                .unwrap();
            }
        }
    }

    dp[m][n]
}
