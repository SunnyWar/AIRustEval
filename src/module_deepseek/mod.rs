use chrono::NaiveDate;

use crate::common::AICodeGenStatus;
use crate::common::CandidateInfo;

// Note: this functions was hand-crafted and cannot change.
// They represent the baseline that each AI has to improve upon.

// add to the vectors as more attempts an this function are made by the AI
pub fn get_candidates() -> CandidateInfo {
    CandidateInfo::new(
        String::from("deepseek R1"),
        vec!["levenshstein distance".to_string()],
        vec![NaiveDate::from_ymd_opt(2025, 1, 25).unwrap()],
        vec![AICodeGenStatus::Ok],
        vec![levenshtein_distance],
    )
}

#[inline(never)]
pub fn levenshtein_distance(s: &str, t: &str) -> usize {
    if s == t {
        return 0;
    }

    let s_chars: Vec<char> = s.chars().collect();
    let t_chars: Vec<char> = t.chars().collect();
    let m = s_chars.len();
    let n = t_chars.len();

    if m == 0 {
        return n;
    }
    if n == 0 {
        return m;
    }

    let mut row: Vec<usize> = (0..=n).collect();

    for i in 1..=m {
        let s_char = unsafe { *s_chars.get_unchecked(i - 1) };
        let mut diagonal = row[0];
        unsafe {
            *row.get_unchecked_mut(0) = i;
        }

        for j in 1..=n {
            let t_char = unsafe { *t_chars.get_unchecked(j - 1) };
            let cost = (s_char != t_char) as usize;

            let old_diagonal = diagonal;
            diagonal = unsafe { *row.get_unchecked(j) };

            let deletion = diagonal + 1;
            let insertion = unsafe { *row.get_unchecked(j - 1) } + 1;
            let substitution = old_diagonal + cost;

            let min_val = deletion.min(insertion).min(substitution);
            unsafe {
                *row.get_unchecked_mut(j) = min_val;
            }
        }
    }

    unsafe { *row.get_unchecked(n) }
}
