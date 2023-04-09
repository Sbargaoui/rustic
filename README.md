# Exercise A: Variables

### Part 1
- [ ] Make a new project named `kata_rust` using cargo
  - See "cargo help" if you forgot the command.
- [ ] Open `Cargo.toml`
  - [ ] Change the version number to `2.3.4` and save the file.  Keep an eye out for that version number in cargo's output when you run it!
- [ ] In `src/main.rs` at the start of the `main()` function:
  - [ ] Declare the variable `weight` and initialize it to `90`
  - [ ] Declare the variable `reps` and initialize it to `6`
- [ ] Change the `println!(...)` at the end of `main()` to:
  - `println!("Sami can do {} reps of {} kilos on the bench press", reps, weight);`
- [ ] Run your program using cargo (see "cargo help" if you forgot the command).
  Some common errors you may hit:
  - Forgot to use `let` to bind a variable
  - Forgot a semicolon `;` at the end of a line

### Part 2

- [ ] After the `println!(...)`, multiply `weight` by `reps` like this:
  - `weight = weight * reps;`
- [ ] Add a second `println!(...)` to the end:
  - `println!("Sami has lift {} kg in total.", weight);`
- [ ] Run your program again using cargo
  - Did you run into an error about mutability? Go back and add `mut` at the right spot on the line where you declare `weight`.
- [ ] Declare a constant named `WEIGHT` and set it to `90` (the type is `i32`).
- [ ] Declare a constant named `REPS` and set it to `6` (also `i32`).
- [ ] Use the constants to initialize `weight` and `reps`
  - Where did you put the constants?  If you put them inside the `main()` function, try moving them up above `main()` to module scope! 
- [ ] Nice. Congratulate yourself on a job well done!  You are a Rust programmer now!

### Extra challenges:
- [ ] Explicitly annotate the variables with the type `i32`
- [ ] Try binding the variables all at once on one line using a pattern (parentheses and commas) -- can you figure out where `mut` goes?
  - [ ] Can you figure out the correct type annotation when you assign them all in one line?  Hint: it will use the same sort of pattern as the variables and values.
- [ ] Instead of changing weight, use the value `weight * reps` directly in the second `println!(...)`
  - What warning does cargo emit when you run your program now? Can you fix it?
- [ ] Add another variable to your program *but don't use it*.
  - What does cargo say when you run your program?
- [ ] Try modifying a constant in `main()` (for example, `WEIGHT = 70`). What does the error look like?


# Exercise B: Functions

### Part 1

- [ ] Create a new function in the `src/main.rs` named  `fn calculate_total_weight()` taking three parameters: weight, reps, series. There are all of type `i32`. 
- [ ] The function should return the total weight, with type `i32` as well.
- [ ] Call the function in the in `main()` and print the result using `println!(...)` function. 

### Part 2
- [ ] Add a test to your function in `src/main.rs` by adding the test of your function in the assert_eq! function (see below)

```rust
#[cfg(test)]
mod tests {
    use super::calculate_total_weight;

    #[test]
    fn test_calculate_total_weight() {
      // You have to complete the asset_eq! function//
        assert_eq!();
    }
}
```

- [ ] Once your test is written, run it using `cargo test`