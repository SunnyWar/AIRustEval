#![recursion_limit = "256"]

mod baseline;
mod module_copilot;
mod module_openai;
mod module_synthaai;
mod module_gemini;
mod module_claude;
mod module_grok;
mod module_watson;

use chrono::NaiveDate;
use core::fmt;
use std::cell::UnsafeCell;
use std::time::Instant;

#[derive(Copy, Clone, PartialEq)]
pub enum AICodeGenStatus {
    Ok,
    CompileError,
    SecondTryOk,
    SecondTryCompileError,
    IncorrectResult,
    AIRefusedToAnswer,
}

type FnAITest = fn(&str, &str) -> usize;

pub struct CandidateInfo {
    pub name: String,
    pub dates: Vec<NaiveDate>,
    pub status: Vec<AICodeGenStatus>,
    pub functions: Vec<FnAITest>,
}

impl CandidateInfo {
    pub fn new(name: String, dates: Vec<NaiveDate>, status: Vec<AICodeGenStatus>, functions: Vec<FnAITest>) -> Self {
        CandidateInfo { name, dates, status, functions }
    }
}

impl fmt::Display for AICodeGenStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            AICodeGenStatus::Ok => "Ok",
            AICodeGenStatus::CompileError => "CompileError",
            AICodeGenStatus::SecondTryOk => "SecondTryOk",
            AICodeGenStatus::SecondTryCompileError => "SecondTryCompileError",
            AICodeGenStatus::IncorrectResult => "IncorrectResult",
            AICodeGenStatus::AIRefusedToAnswer => "AIRefusedToAnswer"
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

fn print_sorted_results(results: Vec<(String, NaiveDate, AICodeGenStatus, usize, u128, String)>) {
    let mut zero_time_results = vec![];
    let mut non_zero_time_results = vec![];

    // Separate results with time 0 and non-zero time
    for result in results {
        if result.2 == AICodeGenStatus::Ok {
            non_zero_time_results.push(result);
        } else {
            zero_time_results.push(result);
        }
    }

    // Sort non-zero time results by time (descending)
    non_zero_time_results.sort_by(|a, b| b.4.cmp(&a.4));

    // Combine the lists, putting zero time results at the top
    zero_time_results.extend(non_zero_time_results);

    // Print header
    println!(
        "| {:<20} | {:<10} | {:<17} | {:<8} | {:<12} | {:<8} |",
        "Module", "Date", "Status", "Result", "Time (ns)", "Speedup"
    );
    println!("{:-<88}", "");

    // Print sorted results
    for result in zero_time_results {
        println!(
            "| {:<20} | {:<12} | {:<17} | {:<8} | {:<12} | {:<8} |",
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

    let modules = vec![
        (baseline::get_candidates()),
        (module_copilot::get_candidates()),
        (module_claude::get_candidates()),
        (module_gemini::get_candidates()),
        (module_openai::get_candidates()),
        (module_synthaai::get_candidates()),
        (module_grok::get_candidates()),
        (module_watson::get_candidates()),
    ];

    let mut results = Vec::new();

    let baseline_result = time_function(modules[0].functions[0], input1, input2);
    results.push((
        modules[0].name.to_string(),
        modules[0].dates[0],
        modules[0].status[0],
        baseline_result.0,
        baseline_result.1,
        "-----".to_string(),
    ));

    (1..modules.len()).for_each(|i| {
        let mod_result = time_function(modules[i].functions[0], input1, input2);
        let speedup = baseline_result.1 as f64 / mod_result.1 as f64;
        results.push((
            modules[i].name.to_string(),
            modules[i].dates[0],
            modules[i].status[0],
            mod_result.0,
            mod_result.1,
            format!("{:.1}x", speedup),
        ));
    });

    print_sorted_results(results);
}
