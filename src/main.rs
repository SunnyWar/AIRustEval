#![recursion_limit = "256"]

mod baseline;
mod module_claude;
mod module_copilot;
mod module_gemini;
mod module_grok;
mod module_openai;
mod module_synthaai;
mod module_watson;

use chrono::NaiveDate;
use core::fmt;
use prettytable::{Cell, Row, Table, format};
use std::cell::UnsafeCell;
use std::time::Instant;

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum AICodeGenStatus {
    Ok,
    CompileError,
    SecondTryOk,
    SecondTryCompileError,
    IncorrectResult,
    AIRefusedToAnswer,
}

type FnAITest = fn(&str, &str) -> usize;

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

fn print_sorted_results(
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
        modules[0].engine_name.to_string(),
        modules[0].function_names[0].to_string(),
        modules[0].dates[0],
        modules[0].status[0],
        baseline_result.0,
        baseline_result.1,
        "-----".to_string(),
    ));

    modules.iter().enumerate().skip(1).for_each(|(_i, module)| {
        for (j, function) in module.functions.iter().enumerate() {
            let mod_result = time_function(*function, input1, input2);
            let speedup = baseline_result.1 as f64 / mod_result.1 as f64;
            results.push((
                module.engine_name.to_string(),
                module.function_names[j].to_string(),
                module.dates[j],
                module.status[j],
                mod_result.0,
                mod_result.1,
                format!("{:.1}x", speedup),
            ));
        }
    });

    print_sorted_results(results);
}
