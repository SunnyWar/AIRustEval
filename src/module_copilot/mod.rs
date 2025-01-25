use chrono::NaiveDate;

use crate::AICodeGenStatus;
use crate::CandidateInfo;

// found here: https://copilot.microsoft.com/
// Microsoft Copilot does not show version number. The only way to know
// that it's been updated is by looking at the release notes here:
// https://learn.microsoft.com/en-us/copilot/microsoft-365/release-notes?tabs=all
// I'll assume, without any other information, that any time there are
// new release notes, that the AI has been updated.

// add to the vectors as more attempts an this function are made by the AI
pub fn get_candidates() -> CandidateInfo {
    CandidateInfo::new(
        String::from("Microsoft Copilot"),
        vec![
            "levenshstein distance".to_string(),
            "levenshstein distance".to_string(),
            "levenshstein distance".to_string(),
        ],
        vec![
            NaiveDate::from_ymd_opt(2025, 1, 2).unwrap(),
            NaiveDate::from_ymd_opt(2025, 1, 13).unwrap(),
            NaiveDate::from_ymd_opt(2025, 1, 25).unwrap(),
        ],
        vec![
            AICodeGenStatus::Ok,
            AICodeGenStatus::Ok,
            AICodeGenStatus::Ok,
        ],
        vec![
            levenshtein_distance,
            levenshtein_distance2,
            levenshtein_distance3,
        ],
    )
}

#[inline(never)]
pub fn levenshtein_distance(s: &str, t: &str) -> usize {
    let m = s.len();
    let n = t.len();

    // If one of the strings is empty, return the length of the other string
    if m == 0 {
        return n;
    }
    if n == 0 {
        return m;
    }

    // Create two rows for the current and previous distances
    let mut prev_row: Vec<usize> = (0..=n).collect();
    let mut curr_row: Vec<usize> = vec![0; n + 1];

    for (i, sc) in s.chars().enumerate() {
        curr_row[0] = i + 1;
        for (j, tc) in t.chars().enumerate() {
            let cost = if sc == tc { 0 } else { 1 };

            curr_row[j + 1] = *[
                prev_row[j + 1] + 1, // Deletion
                curr_row[j] + 1,     // Insertion
                prev_row[j] + cost,  // Substitution
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
pub fn levenshtein_distance2(s: &str, t: &str) -> usize {
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
    for (i, sc) in s.chars().enumerate().take(m) {
        for (j, tc) in t.chars().enumerate().take(n) {
            let cost = if sc == tc { 0 } else { 1 };

            matrix[i + 1][j + 1] = *[
                matrix[i][j + 1] + 1, // Deletion
                matrix[i + 1][j] + 1, // Insertion
                matrix[i][j] + cost,  // Substitution
            ]
            .iter()
            .min()
            .unwrap();
        }
    }

    // The last element of the matrix is the Levenshtein distance
    matrix[m][n]
}

#[inline(never)]
pub fn levenshtein_distance3(s: &str, t: &str) -> usize {
    let m = s.len();
    let n = t.len();
    let mut prev_row: Vec<usize> = (0..=n).collect();
    let mut curr_row = vec![0; n + 1];
    for (i, sc) in s.chars().enumerate() {
        curr_row[0] = i + 1;
        for (j, tc) in t.chars().enumerate() {
            let cost = if sc == tc { 0 } else { 1 };
            curr_row[j + 1] = (prev_row[j + 1] + 1)
                .min(curr_row[j] + 1)
                .min(prev_row[j] + cost);
        }
        std::mem::swap(&mut prev_row, &mut curr_row);
    }
    prev_row[n]
}

#[inline(never)]
pub fn sum_of_divisors(n: u64) -> u64 {
    let mut sum = 0;
    let mut i = 1;

    while i * i <= n {
        if n % i == 0 {
            sum += i;
            if i != n / i {
                sum += n / i;
            }
        }
        i += 1;
    }

    sum
}
