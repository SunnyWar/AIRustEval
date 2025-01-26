use chrono::NaiveDate;

use crate::common::AICodeGenStatus;
use crate::common::CandidateInfo;

// found here: https://chat.deepseek.com/
// choose the "R1" version

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

// AICodeGenStatus::SecondTryOk
#[inline(never)]
pub fn sum_of_divisors(n: u64) -> u64 {
    if n == 0 {
        return 0;
    }

    let mut sum = 1u64;
    let mut m = n;
    let mut count;

    // Handle factor 2 separately
    count = m.trailing_zeros();
    if count > 0 {
        m >>= count;
        sum *= 2u64.pow(count + 1) - 1;
    }

    // Check odd factors
    let mut i = 3u64;
    while i * i <= m {
        count = 0;
        while m % i == 0 {
            count += 1;
            m /= i;
        }
        if count > 0 {
            sum *= (i.pow(count + 1) - 1) / (i - 1);
        }
        i += 2;
    }

    // Handle remaining prime factor
    if m > 1 {
        sum *= m + 1;
    }

    sum
}

#[inline(never)]
pub fn count_primes(n: u64) -> u64 {
    if n < 2 {
        return 0;
    }

    let sqrt_n = (n as f64).sqrt() as u64;
    let primes = sieve(sqrt_n);
    let block_size = (1 << 18).min(sqrt_n.max(1 << 14)); // 256KB block size
    let mut count = 0;

    // Process numbers in [2, n] using segmented sieve
    let mut low = 2;
    let high = n;

    while low <= high {
        let current_high = high.min(low + block_size - 1);
        let mut segment = vec![true; (current_high - low + 1) as usize];

        for &p in &primes {
            let p_squared = p * p;
            let start = ((low + p - 1) / p * p).max(p_squared);

            if start > current_high {
                continue;
            }

            let offset = start - low;
            let end = current_high - low;
            let step = p as usize;

            unsafe {
                let segment_ptr = segment.as_mut_ptr();
                for i in (offset..=end).step_by(step) {
                    *segment_ptr.add(i as usize) = false;
                }
            }
        }

        count += segment.iter().filter(|&&b| b).count() as u64;
        low = current_high + 1;
    }

    count
}

fn sieve(limit: u64) -> Vec<u64> {
    if limit < 2 {
        return Vec::new();
    }

    let mut sieve = vec![true; (limit + 1) as usize];
    sieve[0] = false;
    sieve[1] = false;

    let sqrt_limit = (limit as f64).sqrt() as u64;
    let mut i = 2;
    while i <= sqrt_limit {
        if sieve[i as usize] {
            let mut j = i * i;
            while j <= limit {
                sieve[j as usize] = false;
                j += i;
            }
        }
        i += 1;
    }

    sieve
        .into_iter()
        .enumerate()
        .filter(|(_, is_prime)| *is_prime)
        .map(|(i, _)| i as u64)
        .collect()
}

#[inline(never)]
pub fn fibonacci(n: u64) -> u64 {
    if n == 0 {
        return 0;
    }

    let mut a = 0u64;
    let mut b = 1u64;
    let mut mask = 1u64 << (63 - n.leading_zeros());

    while mask != 0 {
        let c = a.wrapping_mul(b.wrapping_mul(2).wrapping_sub(a));
        let d = a.wrapping_mul(a).wrapping_add(b.wrapping_mul(b));

        if (n & mask) != 0 {
            a = d;
            b = c.wrapping_add(d);
        } else {
            a = c;
            b = d;
        }
        mask >>= 1;
    }

    a
}

// IncorrectResult
#[inline(never)]
pub fn highly_composite(n: u64) -> u64 {
    0
}

#[inline(never)]
pub fn sum_of_proper_divisors(n: u64) -> u64 {
    if n <= 1 {
        return 0;
    }

    let mut sum = 1u64;
    let mut m = n;

    // Handle factor 2 using bitwise operations
    let count = m.trailing_zeros();
    if count > 0 {
        sum *= (1u64 << (count + 1)) - 1;
        m >>= count;
    }

    // Check odd factors with early exit
    let mut i = 3u64;
    while i * i <= m {
        if m % i == 0 {
            let mut exponent = 0u64;
            while m % i == 0 {
                exponent += 1;
                m /= i;
            }
            sum *= (i.pow(exponent as u32 + 1) - 1) / (i - 1);
            if m == 1 {
                break;
            }
        }
        i += 2;
    }

    // Handle remaining prime factor
    if m > 1 {
        sum *= m + 1;
    }

    sum - n
}
