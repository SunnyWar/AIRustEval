#![recursion_limit = "256"]

mod baseline;
mod module1;
mod module2;

use std::cell::UnsafeCell;
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
/// - `f`: The function to be timed, which takes two `&str` inputs and returns a `usize`.
/// - `input1`, `input2`: The input values to be passed to the function.
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
/// let (result, duration) = time_function(baseline::levenshtein_distance, "kitten", "sitting");
/// println!("Result: {}, Time: {} ns", result, duration);
/// ```
#[inline(never)]
pub fn time_function<F>(f: F, input1: &str, input2: &str) -> (usize, u128)
where
    F: Fn(&str, &str) -> usize,
{
    let start = Instant::now();
    let result = f(input1, input2);
    let duration = start.elapsed();
    let result_cell = UnsafeCell::new(result);
    unsafe {
        std::ptr::write_volatile(result_cell.get(), result);
    }
    (result, duration.as_nanos())
}

fn main() {
    let input1 = "To be, or not to be, that is the question:;
                        Whether 'tis nobler in the mind to suffer
                        The slings and arrows of outrageous fortune,
                        Or to take arms against a sea of troubles,
                        And by opposing end them: to die, to sleep
                        No more; and by a sleep, to say we end
                        The heart-ache, and the thousand natural shocks";

    let input2 = "That Flesh is heir to? 'Tis a consummation;
                        Devoutly to be wished. To die, to sleep,
                        To sleep, perchance to Dream; aye, there's the rub,;
                        For in that sleep of death, what dreams may come,
                        When we have shuffled off this mortal coil,
                        Must give us pause.
                         There's the respect
                        That makes Calamity of so long life:
                        For who would bear the Whips and Scorns of time,";

    println!("baseline Started");
    let baseline_result = time_function(
        baseline::levenshtein_distance,
        input1,
        input2,
    );
    println!("baseline Completed");

    println!("{} Started", module1::name());
    let module1_result = time_function(
        module1::levenshtein_distance,
        input1,
        input2,
    );
    println!("{} Completed", module1::name());

    println!("{} Started", module2::name());
    let module2_result = time_function(
        module2::levenshtein_distance,
        input1,
        input2,
    );
    println!("{} Completed", module2::name());

    let module1_speedup = baseline_result.1 as f64 / module1_result.1 as f64;
    let module2_speedup = baseline_result.1 as f64 / module2_result.1 as f64;

    println!(
        "{:<30} | {:<15} | {:<20} | {:<10}",
        "Module", "Result", "Time (ns)", "Speedup"
    );
    println!("{:-<75}", "");
    println!(
        "{:<30} | {:<15} | {:<20} | {:<10}",
        baseline::name(),
        baseline_result.0,
        baseline_result.1,
        "Baseline"
    );
    println!(
        "{:<30} | {:<15} | {:<20} | {:.2}x",
        module1::name(),
        module1_result.0,
        module1_result.1,
        module1_speedup
    );
    println!(
        "{:<30} | {:<15} | {:<20} | {:.2}x",
        module2::name(),
        module2_result.0,
        module2_result.1,
        module2_speedup
    );
}
