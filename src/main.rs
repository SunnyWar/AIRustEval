#![recursion_limit = "256"]

mod common;
mod module_baseline;
mod module_claude;
mod module_copilot;
mod module_gemini;
mod module_grok;
mod module_openai;
mod module_synthaai;
mod module_watson;

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
        (module_baseline::get_candidates()),
        (module_copilot::get_candidates()),
        (module_claude::get_candidates()),
        (module_gemini::get_candidates()),
        (module_openai::get_candidates()),
        (module_synthaai::get_candidates()),
        (module_grok::get_candidates()),
        (module_watson::get_candidates()),
    ];

    let mut results = Vec::new();

    let baseline_result = common::time_function(modules[0].functions[0], input1, input2);
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
            let mod_result = common::time_function(*function, input1, input2);
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

    common::print_sorted_results(results);
}
