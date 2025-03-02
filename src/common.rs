use chrono::NaiveDate;
use core::fmt;
use prettytable::{Cell, Row, Table, format};
use std::cell::UnsafeCell;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::time::{Duration, Instant};

type FnAITest = fn(&str, &str) -> usize;
type FnAITest2 = fn(u64) -> u64;

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum AICodeGenStatus {
    Ok,
    CompileError,
    SecondTryOk,
    SecondTryCompileError,
    IncorrectResult,
    AIRefusedToAnswer,
}

impl fmt::Display for AICodeGenStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            AICodeGenStatus::Ok => "Ok",
            AICodeGenStatus::CompileError => "CompileError",
            AICodeGenStatus::SecondTryOk => "SecondTryOk",
            AICodeGenStatus::SecondTryCompileError => "SecondTryCompileError",
            AICodeGenStatus::IncorrectResult => "IncorrectResult",
            AICodeGenStatus::AIRefusedToAnswer => "AIRefusedToAnswer",
        };
        write!(f, "{}", s)
    }
}

#[derive(Debug)]
pub struct CandidateInfo {
    pub engine_name: String,
    pub function_names: Vec<String>,
    pub dates: Vec<NaiveDate>,
    pub status: Vec<AICodeGenStatus>,
    pub functions: Vec<FnAITest>,
}

impl CandidateInfo {
    pub fn new(
        engine_name: String,
        function_names: Vec<String>,
        dates: Vec<NaiveDate>,
        status: Vec<AICodeGenStatus>,
        functions: Vec<FnAITest>,
    ) -> Self {
        CandidateInfo {
            engine_name,
            function_names,
            dates,
            status,
            functions,
        }
    }
}

#[derive(Debug)]
pub struct CandidateInfo2 {
    pub engine_name: String,
    pub function_names: Vec<String>,
    pub dates: Vec<NaiveDate>,
    pub status: Vec<AICodeGenStatus>,
    pub functions: Vec<FnAITest2>,
}

impl CandidateInfo2 {
    pub fn new(
        engine_name: String,
        function_names: Vec<String>,
        dates: Vec<NaiveDate>,
        status: Vec<AICodeGenStatus>,
        functions: Vec<FnAITest2>,
    ) -> Self {
        CandidateInfo2 {
            engine_name,
            function_names,
            dates,
            status,
            functions,
        }
    }
}

pub fn run_for_duration<F>(f: F, input1: &str, input2: &str, duration_sec: u64) -> (usize, f64)
where
    F: Fn(&str, &str) -> usize,
{
    let duration = Duration::new(duration_sec, 0);
    let start_time = Instant::now();
    let mut run_count = 0;
    let mut total_result = 0;
    let mut result = 0;

    while Instant::now().duration_since(start_time) < duration {
        result = f(input1, input2);
        total_result += result;
        run_count += 1;
        let result_cell = UnsafeCell::new(result);
        unsafe {
            std::ptr::write_volatile(result_cell.get(), result);
        }
    }

    let total_runtime = Instant::now().duration_since(start_time).as_secs_f64();
    (result, run_count as f64 / total_runtime)
}

pub fn run_for_duration2<F>(f: F, input: u64, duration_sec: u64) -> (u64, f64)
where
    F: Fn(u64) -> u64,
{
    let duration = Duration::new(duration_sec, 0);
    let start_time = Instant::now();
    let mut run_count = 0;
    let mut total_result = 0;
    let mut result = 0;

    while Instant::now().duration_since(start_time) < duration {
        result = f(input);
        total_result += result;
        run_count += 1;
        let result_cell = UnsafeCell::new(result);
        unsafe {
            std::ptr::write_volatile(result_cell.get(), result);
        }
    }

    let total_runtime = Instant::now().duration_since(start_time).as_secs_f64();
    (result, run_count as f64 / total_runtime)
}

pub fn print_sorted_results(
    results: Vec<(
        String,
        String,
        NaiveDate,
        AICodeGenStatus,
        usize,
        f64,
        String,
    )>,
) {
    let mut grouped_results: HashMap<String, Vec<_>> = HashMap::new();

    // Group results by function name
    for result in results {
        let function_name = result.1.clone();
        grouped_results
            .entry(function_name)
            .or_insert_with(Vec::new)
            .push(result);
    }

    for (function_name, mut function_results) in grouped_results {
        let mut zero_time_results = vec![];
        let mut non_zero_time_results = vec![];

        // Separate results with time 0 and non-zero time
        for result in function_results {
            if result.3 == AICodeGenStatus::Ok {
                non_zero_time_results.push(result);
            } else {
                zero_time_results.push(result);
            }
        }

        // Sort non-zero time results by time (ascending)
        non_zero_time_results.sort_by(|a, b| a.5.partial_cmp(&b.5).unwrap_or(Ordering::Equal));

        // Combine the lists, putting zero time results at the top
        zero_time_results.extend(non_zero_time_results);

        // Create a table
        let mut table = Table::new();

        // Set the table format
        table.set_format(*format::consts::FORMAT_NO_LINESEP_WITH_TITLE);

        // Add a header row
        table.set_titles(Row::new(vec![
            Cell::new("AI Engine"),
            Cell::new("Date"),
            Cell::new("Status"),
            Cell::new("Result"),
            Cell::new("Iter/Sec"),
            Cell::new("Speedup"),
        ]));

        // Add rows to the table
        for result in zero_time_results {
            table.add_row(Row::new(vec![
                Cell::new(&result.0),
                Cell::new(&format!("{}", result.2)),
                Cell::new(&format!("{:?}", result.3)),
                Cell::new(&result.4.to_string()),
                Cell::new(&format!("{:.2}", result.5)),
                Cell::new(&result.6),
            ]));
        }

        // Print the table for this function name
        println!("Results for function: {}", function_name);
        table.printstd();
        println!("\n");
    }
}

pub fn print_sorted_results2(
    results: Vec<(String, String, NaiveDate, AICodeGenStatus, u64, f64, String)>,
) {
    let mut grouped_results: HashMap<String, Vec<_>> = HashMap::new();

    // Group results by function name
    for result in results {
        let function_name = result.1.clone();
        grouped_results
            .entry(function_name)
            .or_insert_with(Vec::new)
            .push(result);
    }

    for (function_name, mut function_results) in grouped_results {
        let mut zero_time_results = vec![];
        let mut non_zero_time_results = vec![];

        // Separate results with time 0 and non-zero time
        for result in function_results {
            if result.3 == AICodeGenStatus::Ok {
                non_zero_time_results.push(result);
            } else {
                zero_time_results.push(result);
            }
        }

        // Sort non-zero time results by time (ascending)
        non_zero_time_results.sort_by(|a, b| a.5.partial_cmp(&b.5).unwrap_or(Ordering::Equal));

        // Combine the lists, putting zero time results at the top
        zero_time_results.extend(non_zero_time_results);

        // Create a table
        let mut table = Table::new();

        // Set the table format
        table.set_format(*format::consts::FORMAT_NO_LINESEP_WITH_TITLE);

        // Add a header row
        table.set_titles(Row::new(vec![
            Cell::new("AI Engine"),
            Cell::new("Date"),
            Cell::new("Status"),
            Cell::new("Result"),
            Cell::new("Iter/Sec"),
            Cell::new("Speedup"),
        ]));

        // Add rows to the table
        for result in zero_time_results {
            table.add_row(Row::new(vec![
                Cell::new(&result.0),
                Cell::new(&format!("{}", result.2)),
                Cell::new(&format!("{:?}", result.3)),
                Cell::new(&result.4.to_string()),
                Cell::new(&format!("{:.2}", result.5)),
                Cell::new(&result.6),
            ]));
        }

        // Print the table for this function name
        println!("Results for function: {}", function_name);
        table.printstd();
        println!("\n");
    }
}
