#![recursion_limit = "256"]

mod baseline;
mod module_copilot;
mod module_openai;
mod module_synthaai;
mod module_gemini;
mod module_claude;
mod module_grok;

use chrono::NaiveDate;
use core::fmt;
use std::cell::UnsafeCell;
use std::time::Instant;

#[derive(Copy, Clone)]
pub enum AICodeGenStatus {
    Ok,
    CompileError,
    SecondTryOk,
    SecondTryCompileError,
    IncorrectResult,
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

    let baseline_info = baseline::get_candidates();
    let baseline_result = time_function(baseline_info.functions[0], input1, input2);

    let mod_copilot_info = module_copilot::get_candidates();
    let mod_copilot_result = time_function(mod_copilot_info.functions[0], input1, input2);
    let mod_copilot_speedup = baseline_result.1 as f64 / mod_copilot_result.1 as f64;

    let mod_claude_info = module_claude::get_candidates();
    let mod_claude_result = time_function(mod_claude_info.functions[0], input1, input2);
    let mod_claude_speedup = baseline_result.1 as f64 / mod_claude_result.1 as f64;

    let mod_gemini_info = module_gemini::get_candidates();
    let mod_gemini_result = time_function(mod_gemini_info.functions[0], input1, input2);
    let mod_gemini_speedup = baseline_result.1 as f64 / mod_gemini_result.1 as f64;

    let mod_openai_info = module_openai::get_candidates();
    let mod_openai_result = time_function(mod_openai_info.functions[0], input1, input2);
    let mod_openai_speedup = baseline_result.1 as f64 / mod_openai_result.1 as f64;

    let mod_synthaai_info = module_synthaai::get_candidates();
    let mod_synthaai_result = time_function(mod_synthaai_info.functions[0], input1, input2);
    let mod_synthaai_speedup = baseline_result.1 as f64 / mod_synthaai_result.1 as f64;

    let mod_grok_info = module_grok::get_candidates();
    let mod_grok_result = time_function(mod_grok_info.functions[0], input1, input2);
    let mod_grok_speedup = baseline_result.1 as f64 / mod_grok_result.1 as f64;


    let results = vec![
        (
            baseline_info.name,
            baseline_info.dates[0],
            baseline_info.status[0],
            baseline_result.0,
            baseline_result.1,
            "-----".to_string(),
        ),
        (
            mod_copilot_info.name,
            mod_copilot_info.dates[0],
            mod_copilot_info.status[0],
            mod_copilot_result.0,
            mod_copilot_result.1,
            format!("{:.1}x", mod_copilot_speedup),
        ),
        (
            mod_claude_info.name,
            mod_claude_info.dates[0],
            mod_claude_info.status[0],
            mod_claude_result.0,
            mod_claude_result.1,
            format!("{:.1}x", mod_claude_speedup),
        ),
        (
            mod_gemini_info.name,
            mod_gemini_info.dates[0],
            mod_gemini_info.status[0],
            mod_gemini_result.0,
            mod_gemini_result.1,
            format!("{:.1}x", mod_gemini_speedup),
        ),
        (
            mod_openai_info.name,
            mod_openai_info.dates[0],
            mod_openai_info.status[0],
            mod_openai_result.0,
            mod_openai_result.1,
            format!("{:.1}x", mod_openai_speedup),
        ),
        (
            mod_synthaai_info.name,
            mod_synthaai_info.dates[0],
            mod_synthaai_info.status[0],
            mod_synthaai_result.0,
            mod_synthaai_result.1,
            format!("{:.1}x", mod_synthaai_speedup),
        ),
        (
            mod_grok_info.name,
            mod_grok_info.dates[0],
            mod_grok_info.status[0],
            mod_grok_result.0,
            mod_grok_result.1,
            format!("{:.1}x", mod_grok_speedup),
        ),
        // Add more modules here as needed
    ];

    print_sorted_results(results);
}
