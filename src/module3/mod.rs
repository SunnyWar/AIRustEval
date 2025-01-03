pub fn name() -> &'static str {
    "syntha.ai 2 Jan 2025"
}

#[inline(never)]
pub fn levenshtein_distance(s: &str, t: &str) -> usize {
    let (m, n) = (s.len(), t.len());
    let mut dp = vec![0; n + 1];

    for i in 0..=m {
        let mut prev = i;
        for j in 0..=n {
            if i == 0 {
                dp[j] = j;
            } else if j == 0 {
                prev = i;
            } else {
                let cost = if s.chars().nth(i - 1) == t.chars().nth(j - 1) {
                    0
                } else {
                    1
                };
                let temp = dp[j];
                dp[j] = *[prev + cost, dp[j - 1] + 1, dp[j] + 1].iter().min().unwrap();
                prev = temp;
            }
        }
    }

    dp[n]
}