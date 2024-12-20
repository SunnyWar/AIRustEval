# AIRustEval
Evaluates how good various AI's had generating high quality (fast) Rust functions.

The usage is fairly primative.
1. Take the code showed in mod.rs of baseline
2. Paste it in your favorite AI with the following prompt: "Make this function run faster".
3. Take the generated results and put them in a new module, like the ones already here.
4. Change main() so that it runs the new module.
5. Compile and run and see how it does.

Then submit a pull request to add the code sample to this project.
I hope to build up a body of code the shows the project of AI's in generating quality code.

Finally, the existing baseline code was a whim. I'd like to have better examples of code
that AI's find difficul to optimize.