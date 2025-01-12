use chrono::NaiveDate;

use crate::AICodeGenStatus;

pub fn name() -> (&'static str, NaiveDate, AICodeGenStatus) {
    (
        "Grok",
        NaiveDate::from_ymd_opt(2025, 1, 8).unwrap(),
        AICodeGenStatus::Ok,
    )
}

#[inline(never)]
pub fn levenshtein_distance(s: &str, t: &str) -> usize {
    let (m, n) = (s.len(), t.len());

    // We only need to store two rows at a time since we only look at the previous row
    let mut current_row = vec![0; n + 1];
    let mut previous_row = vec![0; n + 1];

    // Initialize the first row
    for j in 0..=n {
        previous_row[j] = j;
    }

    for i in 1..=m {
        // Set the first element of current row
        current_row[0] = i;

        for j in 1..=n {
            let cost = if s.as_bytes()[i - 1] == t.as_bytes()[j - 1] {
                0
            } else {
                1
            };

            current_row[j] = *[
                current_row[j - 1] + 1,     // Insertion
                previous_row[j] + 1,        // Deletion
                previous_row[j - 1] + cost, // Substitution
            ]
            .iter()
            .min()
            .unwrap();
        }

        // Swap rows
        std::mem::swap(&mut current_row, &mut previous_row);
    }

    // The last element of previous_row now holds the distance
    previous_row[n]
}
