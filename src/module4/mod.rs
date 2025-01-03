pub fn name() -> &'static str {
    "gemini 2.0 Flash 2 Jan 2025"
}

#[inline(never)]
pub fn levenshtein_distance(s: &str, t: &str) -> usize {
    if s.is_empty() {
        return t.len();
    }
    if t.is_empty() {
        return s.len();
    }

    let s: Vec<u8> = s.bytes().collect();
    let t: Vec<u8> = t.bytes().collect();

    let m = s.len();
    let n = t.len();

    let mut v0: Vec<usize> = (0..=n).collect();
    let mut v1: Vec<usize> = vec![0; n + 1];

    for i in 0..m {
        v1[0] = i + 1;

        for j in 0..n {
            let cost = if s[i] == t[j] { 0 } else { 1 };
            v1[j + 1] = (v1[j] + 1).min(v0[j + 1] + 1).min(v0[j] + cost);
        }

        std::mem::swap(&mut v0, &mut v1);
    }

    v0[n]
}