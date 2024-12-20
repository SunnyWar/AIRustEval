mod module1;
mod module2;
mod baseline;

use std::time::Instant;

/// Times the execution of a function with the given input and prevents compiler optimization.
///
/// This function measures the execution time of a function `f` with the provided `input`.
/// The use of `#[inline(never)]` prevents the compiler from inlining the function, ensuring
/// it gets executed as intended. Additionally, the `std::ptr::write_volatile` call is used
/// to prevent the compiler from optimizing away the result computation, ensuring accurate
/// timing results.
///
/// # Parameters
/// - `f`: The function to be timed, which takes an `i32` as input and returns an `i32`.
/// - `input`: The input value to be passed to the function.
///
/// # Returns
/// A tuple containing the result of the function execution and the elapsed time in nanoseconds.
///
/// # Safety
/// The `std::ptr::write_volatile` call is used to force the compiler to recognize the result
/// as being used, preventing any optimization that might otherwise skip the computation.
///
/// # Example
/// ```
/// let (result, duration) = time_function(baseline::fibonacci, 42);
/// println!("Result: {}, Time: {} ns", result, duration);
/// ```
#[inline(never)]
pub fn time_function<F>(f: F, input: i32) -> (i32, u128)
where
    F: Fn(i32) -> i32,
{
    let start = Instant::now();
    let mut result = f(input);
    let duration = start.elapsed();
    unsafe {
        std::ptr::write_volatile(&mut result as *mut i32, result);
    }
    (result, duration.as_nanos())
}

fn main() {
    let input = 46; // largest possible or the result overflows

    let baseline_result = time_function(baseline::fibonacci, input);
    let module1_result = time_function(module1::fibonacci, input);
    let module2_result = time_function(module2::fibonacci, input);

    let module1_speedup = baseline_result.1 as f64 / module1_result.1 as f64;
    let module2_speedup = baseline_result.1 as f64 / module2_result.1 as f64;

    println!("{:<30} | {:<15} | {:<20} | {:<10}", "Module", "Result", "Time (ns)", "Speedup");
    println!("{:-<75}", "");
    println!("{:<30} | {:<15} | {:<20} | {:<10}", baseline::name(), baseline_result.0, baseline_result.1, "Baseline");
    println!("{:<30} | {:<15} | {:<20} | {:.2}x", module1::name(), module1_result.0, module1_result.1, module1_speedup);
    println!("{:<30} | {:<15} | {:<20} | {:.2}x", module2::name(), module2_result.0, module2_result.1, module2_speedup);
}
