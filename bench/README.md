# Learn Rust's basic syntax

## :dart: Objective

Get accustomed to Rust syntax

- variable declaration and assignation
- basic types
- compound types
- operator
- statement vs expression
- unit test

## :pencil: Part 1 - Understand main Rust syntax / types

Let's discover Rust's basic syntax and concepts

:bulb: Notes

Take time to read this part\
You can refer to it when writing exercice implementation\
Everything needed is described below.

### Variables and assignations

```rust
let x; // declare "x"
x = 10; // assign 10 to "x"


// declare with assign
let x = 10; // The default integer type in Rust is i32
let y: i8 = -128;

let x = 1.5; // the default float type in Rust is f64
let y: f64 = 2.0;

//cast
let x: i32 = -1200
let y: u16 = 6547
let res: f64 = x as f64 / y as f64


// two equivalent syntaxes
let a = 5i8;
let a : i8 = 5;

// two equivalent syntaxes
let b = 100_000_000;
let b = 100000000;

// casting
let foo = 3_i64;
let bar = foo as i32;
```

```rust
// no changing value
const FOREVER_AGE: u8 = 18;
static LANGUAGE: &str = "Rust";
```

```rust
// mutability
let age = 5; // declare variable
age = 18 //boom, variables are immutable by default

let mut age = 6; // add mut keyword
age = 5; // ok
```

```rust
print!("display line without newline")
println!("display with placeholders {} and new line", 1);

// pattern options
{}: std::fmt::Display trait
{:?} : std::fmt::Debug trait
{:p}  : format the variable as a pointer and prints the memory address that the value points to.
{:032b} means to format as a binary via the std::fmt::Binary trait with 32 zeroes padded on the left

```

```rust
// syntax to specifiy numbers
123_456   // underscore as separator
0x12      // prefix 0x to indicate hex value
0o23      // prefix 0o to indicate octal value
0b0001    // prefix 0b to indicate binary value
b'a'      // A single byte character

// example with formatting print
println!("Base 10 repr:               {}",   69420);
println!("Base 2 (binary) repr:       {:b}", 69420);
println!("Base 8 (octal) repr:        {:o}", 69420);
println!("Base 16 (hexadecimal) repr: {:x}", 69420);
println!("Base 16 (hexadecimal) repr: {:X}", 69420);

```

:pushpin: Remember

- variables are immutable if no `mut` keyword specified
- type can be explicit or infered when possible for compiler
- println! can format types to text

:books: More resources

- [variables and mutability](https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html)
- [formatting options](https://doc.rust-lang.org/stable/std/fmt/index.html#formatting-traits)

## Strings vs slices

```rust
// String literals, not mutable
let x: &str = "hello world!";  // note lowercase syntax "str"

// String
let s: String = "Hello world".to_string(); // camelCase syntax
// another way to build String
let s: String = String::from("Hello world"); //


// concatenation needs mutable String
let mut hello = String::from("Hello, ");
hello.push('w');
hello.push_str("orld!");

// string block
let json = r#"
        {
          "name": "George",
          "age": 27,
          "verified": false
        }
"#;

```

:pushpin: Remember

- String literals are not mutable (stored on stack)
- String is more conveniant than &str but less "performant" (stored on heap)

:books: More resources

- [Slices vs Strings](https://doc.rust-lang.org/book/ch04-03-slices.html)
- [references and borrowing](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html)

## Vectors, arrays, tuples

```rust
// A fixed-size array of four i32 elements
let mut four_ints: [i32; 4] = [1, 2, 3, 4];
four_ints[4] = 9 // boom, index of bound, cannot extend size

// A dynamic array (vector)
let mut vector: Vec<i32> = vec![1, 2, 3, 4]; // vec! is a macro
vector.push(5); // ok, vector have no fixed size

//tuples
let x: (i32, &str, f64) = (1, "hello", 3.4);

// Destructuring `let`
let (a, b, c) = x;
println!("{} {} {}", a, b, c); // 1 hello 3.4
```

:pushpin: Remember

- array have fixed size
- vector have dynamic size (and many methods to manipulate data)
- destructing is commonly used in match
- functions with `!` like `vec![]` are macro. Rust replace it with code at compilation

:books: More resources

- [destructuring](https://doc.rust-lang.org/rust-by-example/flow_control/match/destructuring.html)
- [arrays and slices](https://doc.rust-lang.org/rust-by-example/primitives/array.html)

## Types

```rust

// Struct
struct Point {
        x: i32,
        y: i32,
}
let origin: Point = Point { x: 0, y: 0 };

// A struct with unnamed fields, called a ‘tuple struct’
struct Point2(i32, i32);
let origin2 = Point2(0, 0);

// enum
enum Direction {
    Right,
    Left,
    Up,
    Down,
}
let left = Direction::Left;

// enum with values
enum Movement {
    Right(i32),
    Left(i32),
    Up(i32),
    Down(i32),
}

// enum with struct values
enum Actions {
    StickAround,
    MoveTo { x: i32, y: i32},
}
```

:pushpin: Remember

- Rust do not mix data and behaviour. You don't have "classes" like in Java
- `enum` are powerful and commonly used with `match` operator

:books: More resources

- [structures](https://doc.rust-lang.org/rust-by-example/custom_types/structs.html)
- [enum](https://doc.rust-lang.org/rust-by-example/custom_types/enum.html)

## Optional and Result

Rust have some enum already defined `Option` and `Result`

```rust
// An output can have either Some value or no value/ None.
enum Option<T> { // T is a generic and it can contain any type of value.
    Some(T),
    None,
}

// retrieve an element in collection can be Some or None
let v = vec![10, 20, 30]; // initialization macro
let idx = 0;
match v.get(idx) {
        Some(value) => println!("Value is {}", value),
        None => println!("No value..."),
}
```

```rust
// A result can represent either success/ Ok or failure/ Err.
// T and E are generics. T can contain any type of value, E can be any error.
enum Result<T, E> {
    Ok(T),
    Err(E),
}

// try to convert string to integer can fail
let num = "10";
match num.parse::<i32>() {
        Ok(value) => println!("Num is an integer {}", value),
        Err(e) => println!("Not an integer... {}", e),
}
```

:pushpin: Remember

- There are no Exception in Rust. Either you have a successful operation or an Error
- There are non Null or Void in Rust. Either you have a value or an absence of value

:books: More resources

- [Option](https://doc.rust-lang.org/rust-by-example/error/option_unwrap.html)
- [Result](https://doc.rust-lang.org/rust-by-example/error/result.html)
- [error handling](https://doc.rust-lang.org/rust-by-example/error.html)

## Functions and iterator

```rust
fn is_even(i: i32) -> bool {
    i % 2 == 0
}

// fizzbuzz
if x % 3 == 0 && x % 5 == 0 {
    println!("FizzBuzz")
} else if x % 3 == 0 {
    println!("Fizz")
} else if x % 5 == 0 {
    println!("Buzz")
} else {
    println!("{}", x)
}
```

:warning: There are no null value in Rust

```rust
// The empty tuple () represents the absence of data.

fn whatever() -> () {}
```

```rust
// pattern matching
 match (self % 3, self % 5) {
    (0, 0) => String::from("FizzBuzz"),
    (0, _) => String::from("Fizz"),
    (_, 0) => String::from("Buzz"),
    _ => format!("{}", self),
}

```

```rust
let sum: i32 =
    (0..5)                   // this is an iterator
    .filter(|i| is_even(*i)) // filter with a closure
    .sum();                  // consume the iterator

println!("sum of even numbers is {}", sum);

//nb: vector is not an iterator
let numbers = vec![1,2,3,4];
let even = numbers.into_iter()                  //get iterator from collection
                .filter(|index| index % 2 == 0) // only keep even
                .collect::<Vec<i32>>();         //collect into vector with explicit type
```

:pushpin: Remember

- statements need `;` keyword
- note the absence of `return whatever ;` keyword when evaluating expression
- match operator evaluates all possible values

:books: More resources

- [expression](https://doc.rust-lang.org/rust-by-example/expression.html)
- [functions](https://doc.rust-lang.org/rust-by-example/fn.html)
- [match](https://doc.rust-lang.org/rust-by-example/flow_control/match.html)
- [iterators](https://doc.rust-lang.org/book/ch13-02-iterators.html)

# Exercise A: Variables

### Part 1

- [ ] Make a new project named `kata_rust` using cargo in this directory
  - See "cargo help" if you forgot the command.
- [ ] Open `Cargo.toml`
  - [ ] Change the version number to `2.3.4` and save the file. Keep an eye out for that version number in cargo's output when you run it!
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
  - Where did you put the constants? If you put them inside the `main()` function, try moving them up above `main()` to module scope!
- [ ] Nice. Congratulate yourself on a job well done! You are a Rust programmer now!

### Extra challenges:

- [ ] Explicitly annotate the variables with the type `i32`
- [ ] Try binding the variables all at once on one line using a pattern (parentheses and commas) -- can you figure out where `mut` goes?
  - [ ] Can you figure out the correct type annotation when you assign them all in one line? Hint: it will use the same sort of pattern as the variables and values.
- [ ] Instead of changing weight, use the value `weight * reps` directly in the second `println!(...)`
  - What warning does cargo emit when you run your program now? Can you fix it?
- [ ] Add another variable to your program _but don't use it_.
  - What does cargo say when you run your program?
- [ ] Try modifying a constant in `main()` (for example, `WEIGHT = 70`). What does the error look like?

# Exercise B: Functions

### Part 1

- [ ] Create a new function in the `src/main.rs` named `fn calculate_total_weight()` taking three parameters: weight, reps, series. There are all of type `i32`.
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
