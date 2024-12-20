pub fn name() -> &'static str {
    "Microsoft Copilot 20 Dec 2024"
}

use std::collections::HashMap;

#[inline(never)]
pub fn fibonacci(n: i32) -> i32 {
    fn fibonacci_memo(n: i32, memo: &mut HashMap<i32, i32>) -> i32 {
        if let Some(&val) = memo.get(&n) {
            return val;
        }
        let result = if n <= 1 {
            n
        } else {
            fibonacci_memo(n - 1, memo) + fibonacci_memo(n - 2, memo)
        };
        memo.insert(n, result);
        result
    }

    let mut memo = HashMap::new();
    fibonacci_memo(n, &mut memo)
}
