pub fn name() -> &'static str {
    "GPT-4-turbo 20 Dec 2024"
}

#[inline(never)]
pub fn fibonacci(n: i32) -> i32 {
    if n <= 1 {
        return n;
    }

    let mut prev = 0;
    let mut current = 1;

    for _ in 2..=n {
        let next = prev + current;
        prev = current;
        current = next;
    }

    current
}

