# AIRustEval
Evaluates how good various AI's had generating high quality (fast) Rust functions. This
isn't about readable the code is or how language idiomatic it is. This is about how
much faster the code can be if an AI has a change to review it.

The usage is fairly primative.
1. Take the code showed in mod.rs of baseline
2. Paste it in your favorite AI with the following prompt: "Make this function run faster".
3. Take the generated results and put them in a new module, like the ones already here.
4. Change main() so that it runs the new module.
5. Compile and run and see how it does.

Many AI's will generate code with errors. Given the oppurtunity to fix the errors they
will often generate correct code on the second try. For this reason, the follow heuristic
is being followed.

1. Prompt AI with the following: "Make this code run faster". Then, within the same prompt,
paste the 'baseline' function into the AI prompt block. The 'baseline' function can be found 
at src\baseline\mod.rs
2. Take the generated code and see if it compiles and runs. Be sure to run as --release.
3. There are three possible states: 
    a. Excecutes and the result matches the baseline.
    b. Executes but the result does not match the baseline.
    c. Fails to compile with errors.
3. Excecutes and the result matches the baseline: Mark it as AICodeGenStatus::Ok. No furthur
steps are necessary.
4. Executes but the result does not match the baseline. Mark it as AICodeGenStatus::IncorrectResult. No furthur
steps are necessary.
5. Fails to compile with errors. Copy the compiler error without explanation into the AI prompt
block. It should generate a new function with corrections. Run it and check the result
against the baseline.
6. As before, there are three possibilities.
    a. Excecutes and the result matches the baseline.
    b. Executes but the result does not match the baseline.
    c. Fails to compile with errors.
7. Mark as AICodeGenStatus::SecondTryOk, AICodeGenStatus::SecondTryCompileError, or AICodeGenStatus::IncorrectResult

I hope to build up a body of code the shows the progress of AI's in generating faster code.

TODO:
- the existing baseline code was a whim. I'd like to have better examples of code
that AI's find difficul to optimize.
- an example of hyper-optimized code that the AI has little hope of making better. 

## Results to date:

| Module                      | Result | Time (ns) | Speedup  |
|-----------------------------|--------|-----------|----------|
| Baseline                    | 305    | 20183400  | Baseline |
| syntha.ai 2 Jan 2025        | 303    | 19964600  | 1.01x    |
| GPT-4-turbo 2 Jan 2025      | 305    | 321800    | 62.72x   |
| Microsoft Copilot 2 Jan 2025| 305    | 286400    | 70.47x   |
| gemini 2.0 Flash 2 Jan 2025 | 305    | 276500    | 73.00x   |
| claude 2 Jan 2025           | 305    | 271800    | 74.26x   |

