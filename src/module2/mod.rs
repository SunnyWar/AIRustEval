use chrono::NaiveDate;

use crate::AICodeGenStatus;

pub fn name() -> (&'static str, NaiveDate, AICodeGenStatus) {
    (
        "GPT-4-turbo",
        NaiveDate::from_ymd_opt(2025, 1, 2).unwrap(),
        AICodeGenStatus::Ok,
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
