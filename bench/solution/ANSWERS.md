# Answers to Exercise A

### Part 1

```shell
$ cargo new variables
```

```toml
# Cargo.toml

[package]
name = "variables"
version = "2.3.4"
# ...
```


```rust
// src/main.rs

fn main() {
    let weight = 90;
    let reps = 6;
    println!("Sami can do {} reps of {} kilos on the bench press", reps, weight);
}

```

```shell
$ cargo run
```

### Part 2

```rust
// src/main.rs

fn main() {
    let weight = 90;
    let reps = 6;
    println!("Sami can do {} reps of {} kilos on the bench press", reps, weight);

    weight = weight * reps; // Error
    println!("Sami has lift {} kg in total.", weight)

}
```


```rust
const WEIGHT: i32 = 90;
const REPS: i32 = 6;

fn main() {
    let mut weight = WEIGHT;
    let reps = REPS;
    println!("Sami can do {} reps of {} kilos on the bench press", reps, weight);

    weight = weight * reps; 
    println!("Sami has lift {} kg in total.", weight)
}
```

### Extra challenges

- Explicitly annotate the variables with the type `i32`

```rust
const WEIGHT: i32 = 90;
const REPS: i32 = 6;

fn main() {
    let mut weight: i32 = WEIGHT;
    let reps: i32 = REPS;
    println!("Sami can do {} reps of {} kilos on the bench press", reps, weight);

    weight = weight * reps; 
    println!("Sami has lift {} kg in total.", weight)
}
```

- Try binding the variables all at once on one line using a pattern (parentheses and commas) -- can you figure out where `mut` goes?

```rust
const WEIGHT: i32 = 90;
const REPS: i32 = 6;

fn main() {
    let (mut weight, reps) = (WEIGHT, REPS);
    println!("Sami can do {} reps of {} kilos on the bench press", reps, weight);

    weight = weight * reps; 
    println!("Sami has lift {} kg in total.", weight)
}

```

- Can you figure out the correct type annotation when you assign them all in one line? Hint: it will use the same sort of pattern as the variables and values.

```rust
const WEIGHT: i32 = 90;
const REPS: i32 = 6;

fn main() {
    let (mut weight, reps): (i32, i32) = (WEIGHT, REPS);
    println!("Sami can do {} reps of {} kilos on the bench press", reps, weight);

    weight = weight * reps; 
    println!("Sami has lift {} kg in total.", weight)
}
```

- Instead of changing weight, use the value `weight * reps` directly in the second `println!(...)`
  - What does cargo say when you run your program?  

It gives this warning:

```
warning: variable does not need to be mutable
```

- Add another variable to your program *but don't use it*.
  - What does cargo say when you run your program?  

It gives this warning if my unused variable is named `jet`:

```
warning: unused variable: `jet`
```

- Try modifying a constant in main() (for example, `WEIGHT = 80`). What does the error look like?

```
error[E0070]: invalid left-hand side of assignment
 --> src/main.rs:7:12
  |
5 |     WEIGHT = 80;
  |     ------------ ^
  |     |
  |     cannot assign to this expression
```


# Answers to Exercise B

### Part 1

```rust
const WEIGHT: i32 = 90;
const REPS: i32 = 6;

fn main() {
    let (weight, reps): (i32, i32) = (WEIGHT, REPS);
    println!("Sami can do {} reps of {} kilos on the bench press", reps, weight);
    let total_weight  = calculate_total_weight(weight, reps, 5);
    println!("Sami can do {} kilos on the bench press", total_weight);

}

fn calculate_total_weight(weight: i32, reps: i32, series: i32) -> i32 {
    weight * reps * series
}
```

### Part 2

You can add a unit test by adding the `#[cfg(test)]` flag.  
Once your test in written, you can run it using `cargo test`


```rust
const WEIGHT: i32 = 90;
const REPS: i32 = 6;

fn main() {
    let (weight, reps): (i32, i32) = (WEIGHT, REPS);
    println!("Sami can do {} reps of {} kilos on the bench press", reps, weight);
    let total_weight  = calculate_total_weight(weight, reps, 5);
    println!("Sami can do {} kilos on the bench press", total_weight);

}

fn calculate_total_weight(weight: i32, reps: i32, series: i32) -> i32 {
    weight * reps * series
}

#[cfg(test)]
mod tests {
    use super::calculate_total_weight;

    #[test]
    fn test_calculate_total_weight() {
        assert_eq!(calculate_total_weight(90, 6, 5), 2700);
    }
}
```