#![recursion_limit = "256"]

mod common;
mod module_baseline;
mod module_claude;
mod module_copilot;
mod module_deepseek;
mod module_gemini;
mod module_grok;
mod module_openai;
mod module_synthaai;
mod module_watson;

fn main() {
    let fun_duration = 2;
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
        module_baseline::get_candidates(),
        module_copilot::get_candidates(),
        module_claude::get_candidates(),
        module_gemini::get_candidates(),
        module_openai::get_candidates(),
        module_synthaai::get_candidates(),
        module_grok::get_candidates(),
        module_watson::get_candidates(),
        module_deepseek::get_candidates(),
    ];

    let mut results = Vec::new();

    let baseline_result =
        common::run_for_duration(modules[0].functions[0], input1, input2, fun_duration);
    results.push((
        modules[0].engine_name.to_string(),
        modules[0].function_names[0].to_string(),
        modules[0].dates[0],
        modules[0].status[0],
        baseline_result.0,
        baseline_result.1,
        "-----".to_string(),
    ));

    get_fun_results(
        fun_duration,
        input1,
        input2,
        modules,
        &mut results,
        baseline_result,
    );

    common::print_sorted_results(results);

    // ---------------------------------------------------------------------------
    //                type 2 functions
    // ---------------------------------------------------------------------------

    let modules2 = vec![
        module_baseline::get_candidates2(),
        module_copilot::get_candidates2(),
        module_deepseek::get_candidates2(),
        module_openai::get_candidates2(),
        module_claude::get_candidates2(),
        module_gemini::get_candidates2(),
    ];

    let mut results2 = Vec::new();

    let baseline_result2 = common::run_for_duration2(modules2[0].functions[0], 1000, fun_duration);
    results2.push((
        modules2[0].engine_name.to_string(),
        modules2[0].function_names[0].to_string(),
        modules2[0].dates[0],
        modules2[0].status[0],
        baseline_result2.0,
        baseline_result2.1,
        "-----".to_string(),
    ));

    get_fun_results2(
        fun_duration,
        1000,
        modules2,
        &mut results2,
        baseline_result2,
    );

    common::print_sorted_results2(results2);
}

fn get_fun_results(
    fun_duration: u64,
    input1: &str,
    input2: &str,
    modules: Vec<common::CandidateInfo>,
    results: &mut Vec<(
        String,
        String,
        chrono::NaiveDate,
        common::AICodeGenStatus,
        usize,
        f64,
        String,
    )>,
    baseline_result: (usize, f64),
) {
    modules.iter().enumerate().skip(1).for_each(|(_i, module)| {
        for (j, function) in module.functions.iter().enumerate() {
            let mod_result = common::run_for_duration(*function, input1, input2, fun_duration);

            let speedup = if baseline_result.1 > 0.0 {
                mod_result.1 / baseline_result.1
            } else {
                0.0 // Or handle this error case differently
            };

            if mod_result.0 == 0 {
                results.push((
                    module.engine_name.to_string(),
                    module.function_names[j].to_string(),
                    module.dates[j],
                    module.status[j],
                    mod_result.0,
                    0.0,
                    "none".to_string(),
                ));
            } else {
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
        }
    });
}

fn get_fun_results2(
    fun_duration: u64,
    input: u64,
    modules: Vec<common::CandidateInfo2>,
    results: &mut Vec<(
        String,
        String,
        chrono::NaiveDate,
        common::AICodeGenStatus,
        u64,
        f64,
        String,
    )>,
    baseline_result: (u64, f64),
) {
    modules.iter().enumerate().skip(1).for_each(|(_i, module)| {
        for (j, function) in module.functions.iter().enumerate() {
            let mod_result = common::run_for_duration2(*function, input, fun_duration);
            let speedup = mod_result.1 / baseline_result.1;

            if mod_result.0 == 0 {
                results.push((
                    module.engine_name.to_string(),
                    module.function_names[j].to_string(),
                    module.dates[j],
                    module.status[j],
                    mod_result.0,
                    0.0,
                    "none".to_string(),
                ));
            } else {
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
        }
    });
}
