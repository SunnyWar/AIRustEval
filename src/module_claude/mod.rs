use chrono::NaiveDate;

use crate::AICodeGenStatus;
use crate::CandidateInfo;

// found here: https://claude.ai/
// version can be found by asking claude "what version are you?"

// add to the vectors as more attempts an this function are made by the AI
pub fn get_candidates() -> CandidateInfo {
    CandidateInfo::new(
        String::from("claude 3.5 Sonnet"),
        vec![NaiveDate::from_ymd_opt(2025, 1, 2).unwrap()],
        vec![AICodeGenStatus::Ok],
        vec![levenshtein_distance],
    )
}

#[inline(never)]
pub fn levenshtein_distance(s: &str, t: &str) -> usize {
    // Handle empty string cases first
    if s.is_empty() {
        return t.len();
    }
    if t.is_empty() {
        return s.len();
    }

    // Get the byte slices - this is safe for ASCII and we'll handle UTF-8 specially
    let s_bytes = s.as_bytes();
    let t_bytes = t.as_bytes();

    // Use the shorter string as the column length to minimize memory usage
    let (row_bytes, col_bytes) = if s_bytes.len() < t_bytes.len() {
        (t_bytes, s_bytes)
    } else {
        (s_bytes, t_bytes)
    };

    // Only allocate a single vector for current and previous rows
    let mut prev_row = Vec::with_capacity(col_bytes.len() + 1);
    let mut curr_row = Vec::with_capacity(col_bytes.len() + 1);

    // Initialize the first row
    prev_row.extend(0..=col_bytes.len());
    curr_row.resize(col_bytes.len() + 1, 0);

    // Process each character in the row string
    for (i, row_byte) in row_bytes.iter().enumerate() {
        curr_row[0] = i + 1;

        // Process each character in the column string
        for (j, col_byte) in col_bytes.iter().enumerate() {
            let substitution_cost = if row_byte == col_byte { 0 } else { 1 };

            curr_row[j + 1] = min3(
                curr_row[j] + 1,                 // insertion
                prev_row[j + 1] + 1,             // deletion
                prev_row[j] + substitution_cost, // substitution
            );
        }

        // Swap rows
        std::mem::swap(&mut curr_row, &mut prev_row);
    }

    // Return the last element from the previous row (due to the final swap)
    prev_row[col_bytes.len()]
}

// Specialized minimum function for three values
#[inline(always)]
fn min3(a: usize, b: usize, c: usize) -> usize {
    std::cmp::min(a, std::cmp::min(b, c))
}
