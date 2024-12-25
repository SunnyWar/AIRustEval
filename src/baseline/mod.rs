pub fn name() -> &'static str {
    "Baseline"
}

#[inline(never)]
pub fn levenshtein_distance<T: Eq>(s: &[T], t: &[T]) -> usize {
    // If s is empty, the distance is the number of characters in t
    if s.is_empty() {
        return t.len();
    }

    // If t is empty, the distance is the number of characters in s
    if t.is_empty() {
        return s.len();
    }

    let (a, s_prime) = s.split_first().unwrap();
    let (b, t_prime) = t.split_first().unwrap();

    if a == b {
        // If the first characters are the same, they can be ignored
        levenshtein_distance(s_prime, t_prime)
    } else {
        // Otherwise try all three possible actions and select the best one
        1 + std::cmp::min(
            std::cmp::min(
                levenshtein_distance(s_prime, t), // Character is deleted (a deleted)
                levenshtein_distance(s, t_prime), // Character is inserted (b inserted)
            ),
            levenshtein_distance(s_prime, t_prime), // Character is replaced (a replaced with b)
        )
    }
}
