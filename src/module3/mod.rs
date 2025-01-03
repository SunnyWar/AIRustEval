pub fn name() -> &'static str {
    "syntha.ai 2 Jan 2025"
}

#[inline(never)]
pub fn levenshtein_distance(s: &str, t: &str) -> usize {
    let (m, n) = (s.len(), t.len());
    let mut dp = vec![vec![0; n + 1]; m + 1];

    for i in 0..=m {
        for j in 0..=n {
            if i == 0 {
                dp[i][j] = j;
            } else if j == 0 {
                dp[i][j] = i;
            } else {
                let cost = if s.chars().nth(i - 1) == t.chars().nth(j - 1) {
                    0
                } else {
                    1
                };
                dp[i][j] = *[
                    dp[i - 1][j] + 1,      // Deletion
                    dp[i][j - 1] + 1,      // Insertion
                    dp[i - 1][j - 1] + cost, // Substitution
                ].iter().min().unwrap();
            }
        }
    }

    dp[m][n]
}
