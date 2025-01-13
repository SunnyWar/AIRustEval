use chrono::NaiveDate;

use crate::AICodeGenStatus;
use crate::CandidateInfo;

// found here: https://www.ibm.com/watsonx
// requires an account and doesn't work very well so will likely not try it again.

// add to the vectors as more attempts an this function are made by the AI
pub fn get_candidates() -> CandidateInfo {
    CandidateInfo::new(
        String::from("watson"),
        vec!["levenshstein distance".to_string()],
        vec![NaiveDate::from_ymd_opt(2025, 1, 10).unwrap()],
        vec![AICodeGenStatus::AIRefusedToAnswer],
        vec![levenshtein_distance],
    )
}

#[inline(never)]
pub fn levenshtein_distance(_s: &str, _t: &str) -> usize {
    0
}
