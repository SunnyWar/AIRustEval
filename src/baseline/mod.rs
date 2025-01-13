use crate::AICodeGenStatus;
use crate::CandidateInfo;
use chrono::NaiveDate;
use std::vec;

// add to the vectors as more attempts an this function are made by the AI
pub fn get_candidates() -> CandidateInfo {
    CandidateInfo::new(
        String::from("Baseline"),
        vec![NaiveDate::from_ymd_opt(2025, 1, 2).unwrap()],
        vec![AICodeGenStatus::Ok],
        vec![levenshtein_distance],
    )
}

#[inline(never)]
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
