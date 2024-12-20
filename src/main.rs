mod module1;
mod module2;
mod baseline;

use std::time::Instant;

fn time_function<F>(f: F, input: i32) -> (i32, u128)
where
    F: Fn(i32) -> i32,
{
    let start = Instant::now();
    let result = f(input);
    let duration = start.elapsed();
    (result, duration.as_nanos())
}

fn main() {
    let input = 42;

    let baseline_result = time_function(baseline::process, input);
    let module1_result = time_function(module1::process, input);
    let module2_result = time_function(module2::process, input);

    let module1_speedup = baseline_result.1 as f64 / module1_result.1 as f64;
    let module2_speedup = baseline_result.1 as f64 / module2_result.1 as f64;

    println!("{:<30} | {:<15} | {:<20} | {:<10}", "Module", "Result", "Time (ns)", "Speedup");
    println!("{:-<60}", "");
    println!("{:<30} | {:<15} | {:<20} | {:<10}", baseline::name(), baseline_result.0, baseline_result.1, "Baseline");
    println!("{:<30} | {:<15} | {:<20} | {:.2}x", module1::name(), module1_result.0, module1_result.1, module1_speedup);
    println!("{:<30} | {:<15} | {:<20} | {:.2}x", module2::name(), module2_result.0, module2_result.1, module2_speedup);
}
