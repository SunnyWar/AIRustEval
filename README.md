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

Then submit a pull request to add the code sample to this project.
I hope to build up a body of code the shows the progress of AI's in generating faster code.

TODO:
- the existing baseline code was a whim. I'd like to have better examples of code
that AI's find difficul to optimize.
- an example of hyper-optimized code that the AI has little hope of making better. 

## Results to date:

| Module                        | Result | Time (ns) | Speedup  |
|-------------------------------|--------|-----------|----------|
| Baseline                      | 306    | 24243400  | Baseline |
| Microsoft Copilot 02 Jan 2025 | 306    | 320100    | 75.74x   |
| GPT-4-turbo 2 Jan 2025        | 306    | 388900    | 62.34x   |

