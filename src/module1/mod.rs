pub fn name() -> &'static str {
    "Microsoft Copilot 20 Dec 2024"
}

pub fn levenshtein_distance<T: Eq>(s: &[T], t: &[T]) -> usize {
    let m = s.len();
    let n = t.len();

    // Create a 2D vector to store distances
    let mut dp = vec![vec![0; n + 1]; m + 1];

    // Initialize the base cases
    for i in 0..=m {
        dp[i][0] = i; // Distance to empty string t
    }
    for j in 0..=n {
        dp[0][j] = j; // Distance to empty string s
    }

    // Fill the DP table
    for i in 1..=m {
        for j in 1..=n {
            if s[i - 1] == t[j - 1] {
                dp[i][j] = dp[i - 1][j - 1]; // No change needed
            } else {
                dp[i][j] = 1 + std::cmp::min(
                    std::cmp::min(dp[i - 1][j], dp[i][j - 1]), // Deletion or insertion
                    dp[i - 1][j - 1],                          // Substitution
                );
            }
        }
    }

    // The final answer is in dp[m][n]
    dp[m][n]
}
