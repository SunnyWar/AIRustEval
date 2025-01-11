#![recursion_limit = "256"]

mod baseline;
mod module1;
mod module2;
mod module3;
mod module4;
mod module5;
mod module6;

use chrono::NaiveDate;
use core::fmt;
use std::cell::UnsafeCell;
use std::time::Instant;

pub enum AICodeGenStatus {
    Ok,
    CompileError,
    SecondTryOk,
    SecondTryCompileError,
    IncorrectResult,
}

impl fmt::Display for AICodeGenStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            AICodeGenStatus::Ok => "Ok",
            AICodeGenStatus::CompileError => "CompileError",
            AICodeGenStatus::SecondTryOk => "SecondTryOk",
            AICodeGenStatus::SecondTryCompileError => "SecondTryCompileError",
            AICodeGenStatus::IncorrectResult => "IncorrectResult",
        };
        write!(f, "{}", s)
    }
}

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

fn print_sorted_results(results: Vec<(&str, NaiveDate, AICodeGenStatus, usize, u128, String)>) {
    let mut sorted_results = results;

    // Sort results by time (descending)
    sorted_results.sort_by(|a, b| b.4.cmp(&a.4));

    // Print header
    println!(
        "| {:<20} | {:<10} | {:<10} | {:<8} | {:<12} | {:<8} |",
        "Module", "Date", "Status", "Result", "Time (ns)", "Speedup"
    );
    println!("{:-<88}", "");

    // Print sorted results
    for result in sorted_results {
        println!(
            "| {:<20} | {:<12} | {:<10} | {:<8} | {:<12} | {:<8} |",
            result.0,
            result.1,
            format!("{}", result.2),
            result.3,
            result.4,
            result.5
        );
    }
}

fn main() {
    let input1 = "To be, or not to be, that is the question:
                        Whether 'tis nobler in the mind to suffer
                        The slings and arrows of outrageous fortune,
                        Or to take arms against a sea of troubles,
                        And by opposing end them: to die, to sleep
                        No more; and by a sleep, to say we end
                        The heart-ache, and the thousand natural shocks";

    let input2 = "That Flesh is heir to? 'Tis a consummation:
                        Devoutly to be wished. To die, to sleep,
                        To sleep, perchance to Dream; aye, there's the rub;
                        For in that sleep of death, what dreams may come,
                        When we have shuffled off this mortal coil,
                        Must give us pause.
                         There's the respect
                        That makes Calamity of so long life:
                        For who would bear the Whips and Scorns of time,";

    let baseline_result = time_function(baseline::levenshtein_distance, input1, input2);

    let module1_result = time_function(module1::levenshtein_distance, input1, input2);
    let module1_speedup = baseline_result.1 as f64 / module1_result.1 as f64;

    let module2_result = time_function(module2::levenshtein_distance, input1, input2);
    let module2_speedup = baseline_result.1 as f64 / module2_result.1 as f64;

    let module3_result = time_function(module3::levenshtein_distance, input1, input2);
    let module3_speedup = baseline_result.1 as f64 / module3_result.1 as f64;

    let module4_result = time_function(module4::levenshtein_distance, input1, input2);
    let module4_speedup = baseline_result.1 as f64 / module4_result.1 as f64;

    let module5_result = time_function(module5::levenshtein_distance, input1, input2);
    let module5_speedup = baseline_result.1 as f64 / module5_result.1 as f64;

    let module6_result = time_function(module6::levenshtein_distance, input1, input2);
    let module6_speedup = baseline_result.1 as f64 / module6_result.1 as f64;

    let results = vec![
        (
            baseline::name().0,
            baseline::name().1,
            baseline::name().2,
            baseline_result.0,
            baseline_result.1,
            "-----".to_string(),
        ),
        (
            module1::name().0,
            module1::name().1,
            module1::name().2,
            module1_result.0,
            module1_result.1,
            format!("{:.1}x", module1_speedup),
        ),
        (
            module2::name().0,
            module2::name().1,
            module2::name().2,
            module2_result.0,
            module2_result.1,
            format!("{:.1}x", module2_speedup),
        ),
        (
            module3::name().0,
            module3::name().1,
            module3::name().2,
            module3_result.0,
            module3_result.1,
            format!("{:.1}x", module3_speedup),
        ),
        (
            module4::name().0,
            module4::name().1,
            module4::name().2,
            module4_result.0,
            module4_result.1,
            format!("{:.1}x", module4_speedup),
        ),
        (
            module5::name().0,
            module5::name().1,
            module5::name().2,
            module5_result.0,
            module5_result.1,
            format!("{:.1}x", module5_speedup),
        ),
        (
            module6::name().0,
            module6::name().1,
            module6::name().2,
            module6_result.0,
            module6_result.1,
            format!("{:.2}x", module6_speedup),
        ),
        // Add more modules here as needed
    ];

    print_sorted_results(results);
}
