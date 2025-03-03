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

PROMPT USED: "You are a highly skilled Rust developer. Your task is to optimize the following Rust function for maximum speed and efficiency. The code will run on a x86-x64 system with an AMD Ryzen 7 processor. Readability of the code is not important. Please provide the optimized code. Here is the function that needs optimization:"

1. Prompt AI with that pasted the 'baseline' function into the AI prompt block. The 'baseline' function can be found 
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

| AI Engine          | Date       | Status            | Result | Iter/Sec | Speedup |
|--------------------|------------|-------------------|--------|----------|---------|
| watson             | 2025-01-10 | AIRefusedToAnswer | 0      | 0.00     | none    |
| syntha.ai          | 2025-01-02 | Ok                | 305    | 29.62    | 0.6x    |
| Baseline           | 2025-01-02 | Ok                | 305    | 48.73    | -----   |
| Microsoft Copilot  | 2025-01-13 | Ok                | 305    | 613.58   | 12.6x   |
| Microsoft Copilot  | 2025-01-25 | Ok                | 305    | 1690.15  | 34.7x   |
| ChatGPT, version 2 | 2025-01-02 | Ok                | 305    | 1769.60  | 36.3x   |
| claude 3.5 Sonnet  | 2025-01-02 | Ok                | 305    | 1935.41  | 39.7x   |
| ChatGPT, version 2 | 2025-01-25 | Ok                | 305    | 1938.75  | 39.8x   |
| deepseek R1        | 2025-01-25 | Ok                | 305    | 2053.37  | 42.1x   |
| gemini 2.0 Flash   | 2025-01-02 | Ok                | 305    | 2083.08  | 42.8x   |
| Grock 2            | 2025-01-08 | Ok                | 305    | 2096.83  | 43.0x   |
| Microsoft Copilot  | 2025-01-02 | Ok                | 305    | 2535.17  | 52.0x   |


