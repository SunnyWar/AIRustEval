use chrono::NaiveDate;

use crate::AICodeGenStatus;

pub fn name() -> (&'static str, NaiveDate, AICodeGenStatus) {
    (
        "Microsoft Copilot",
        NaiveDate::from_ymd_opt(2025, 1, 2).unwrap(),
        AICodeGenStatus::Ok,
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
