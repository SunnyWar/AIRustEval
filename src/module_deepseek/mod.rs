use crate::AICodeGenStatus;
use crate::CandidateInfo;
use chrono::NaiveDate;
use std::vec;

// Note: this functions was hand-crafted and cannot change.
// They represent the baseline that each AI has to improve upon.

// add to the vectors as more attempts an this function are made by the AI
pub fn get_candidates() -> CandidateInfo {
    CandidateInfo::new(
        String::from("deepseek V3"),
        vec!["levenshstein distance".to_string()],
        vec![NaiveDate::from_ymd_opt(2025, 1, 25).unwrap()],
        vec![AICodeGenStatus::Ok],
        vec![levenshtein_distance],
    )
}

#[inline(never)]
pub fn levenshtein_distance(s: &str, t: &str) -> usize {
    let s_chars: Vec<char> = s.chars().collect();
    let t_chars: Vec<char> = t.chars().collect();
    let m = s_chars.len();
    let n = t_chars.len();

    // Use a 1D vector to store the distances
    let mut matrix = vec![0; (m + 1) * (n + 1)];

    // Initialize the first row and column of the matrix
    for i in 0..=m {
        matrix[i * (n + 1)] = i;
    }
    for j in 0..=n {
        matrix[j] = j;
    }

    // Compute the Levenshtein distance
    for i in 1..=m {
        for j in 1..=n {
            let cost = if s_chars[i - 1] == t_chars[j - 1] {
                0
            } else {
                1
            };

            let deletion = matrix[(i - 1) * (n + 1) + j] + 1;
            let insertion = matrix[i * (n + 1) + (j - 1)] + 1;
            let substitution = matrix[(i - 1) * (n + 1) + (j - 1)] + cost;

            matrix[i * (n + 1) + j] = deletion.min(insertion).min(substitution);
        }
    }

    // The last element of the matrix is the Levenshtein distance
    matrix[m * (n + 1) + n]
}
