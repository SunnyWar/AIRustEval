pub fn name() -> &'static str {
    "Baseline"
}

pub fn process(input: i32) -> i32 {
    fn fibonacci(n: i32) -> i32 {
        if n <= 1 {
            return n;
        }
        fibonacci(n - 1) + fibonacci(n - 2)
    }

    // Adding complexity with nested loops
    let mut result = 0;
    for i in 0..input {
        result += fibonacci(i);
        for j in 0..i {
            result += j * fibonacci(j);
        }
    }
    result
}
