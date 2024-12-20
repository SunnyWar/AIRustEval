pub fn name() -> &'static str {
    "Baseline"
}

// naive implementation
#[inline(never)]
pub fn fibonacci(n: i32) -> i32 {
    if n <= 1 {
        return n;
    }
    return fibonacci(n - 1) + fibonacci(n - 2);
}
