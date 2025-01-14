use chrono::NaiveDate;
use core::fmt;
use prettytable::{Cell, Row, Table, format};
use std::cell::UnsafeCell;
use std::time::Instant;

type FnAITest = fn(&str, &str) -> usize;

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

pub fn print_sorted_results(
    results: Vec<(
        String,
        String,
        NaiveDate,
        AICodeGenStatus,
        usize,
        u128,
        String,
    )>,
) {
    let mut zero_time_results = vec![];
    let mut non_zero_time_results = vec![];

    // Separate results with time 0 and non-zero time
    for result in results {
        if result.3 == AICodeGenStatus::Ok {
            non_zero_time_results.push(result);
        } else {
            zero_time_results.push(result);
        }
    }

    // Sort non-zero time results by time (ascending)
    non_zero_time_results.sort_by(|a, b| b.5.cmp(&a.5));

    // Combine the lists, putting zero time results at the top
    zero_time_results.extend(non_zero_time_results);

    // Create a table
    let mut table = Table::new();

    // Set the table format
    table.set_format(*format::consts::FORMAT_NO_LINESEP_WITH_TITLE);

    // Add a header row
    table.set_titles(Row::new(vec![
        Cell::new("AI Engine"),
        Cell::new("Function Name"),
        Cell::new("Date"),
        Cell::new("Status"),
        Cell::new("Result"),
        Cell::new("Time (ns)"),
        Cell::new("Speedup"),
    ]));

    // Add rows to the table
    for result in zero_time_results {
        table.add_row(Row::new(vec![
            Cell::new(&result.0),
            Cell::new(&result.1),
            Cell::new(&format!("{}", result.2)),
            Cell::new(&format!("{:?}", result.3)),
            Cell::new(&result.4.to_string()),
            Cell::new(&result.5.to_string()),
            Cell::new(&result.6),
        ]));
    }

    // Print the table
    table.printstd();
}
