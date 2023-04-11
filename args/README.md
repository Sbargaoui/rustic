# Learn how to parse arguments

In order to interact with our cli, we need to provide and retrieve arguments

## :dart: Objectives

- interact with the context
- add dependencies
- parse arguments
- manage errors
- use modules
- type conversion

## :pencil: Retrieve cli arguments

Rust already imports commonly used symbols like Vec, String, Option and Result.\
For other symbols, you must import module with `use` keyword

```rust
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
}
```

```bash
$ cargo run

     Running `/your/path/target/debug/rustic`
["/your/path/target/debug/rustic"]

$ cargo run -- foo bar 1

     Running `/your/path/target/debug/rustic`
["/your/path/target/debug/rustic", "foo", "bar", 1]
```

:pushpin: Remember

- Every argument after `--` is passed to the execution context. \
  Avoid mixing arguments with `cargo run` and program options (ex: cargo run --help)\
  Equivalent to `./target/debug/rustic foo bar 1`
- Arguments collected are Strings natively - think about casting them if needed.

:bulb: Common libraries provided in Rust's standard library

- fmt: [formatting and printing](https://doc.rust-lang.org/std/fmt/)
- env: [inspect and manipulate process environment](https://doc.rust-lang.org/std/env/)
- io: [input and output](https://doc.rust-lang.org/std/io/)
- path: [working with path abstraction](https://doc.rust-lang.org/stable/std/path/)
- fs: [filesystem](https://doc.rust-lang.org/stable/std/fs/)

:books: More resources

- [std library](https://doc.rust-lang.org/std/)
- [std modules](https://doc.rust-lang.org/std/#modules)
- [documentation about prelude](https://doc.rust-lang.org/std/prelude/index.html)

## :pencil: Manage Errors

Update your code to greet with argument passed to our app

```rust
fn main() {
    let args: Vec<String> = env::args().collect();

    greets(&args[1])
}

fn greets(name: &str) {
    println!("Hello {} 🦀 !", name);
}

```

```bash
$ cargo run -- Joe

Running `/your/path/target/debug/args Joe`
Hello Joe 🦀 !
```

What happens if no argument is provided ?

<details>
<summary>&#128073; Check by yourself to see error</summary>

```bash
$ cargo run --

thread 'main' panicked at 'index out of bounds: the len is 1 but the index is 1'
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

</details>

### We need to manage errors gracefully

Vectors implements `get` api to retrieve element by position ([documentation](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.get))

```rust
// from api doc
let v = [10, 40, 30];
assert_eq!(Some(&40), v.get(1));
assert_eq!(Some(&[10, 40][..]), v.get(0..2));
assert_eq!(None, v.get(3));
assert_eq!(None, v.get(0..4));
```

Either we have an argument provided or none

Optional can have a value `Some(val)` or `None`

```rust
enum Option<T> {
    Some(T),
    None,
}
```

Many ways to manage our code :

```rust
// manage both cases
match args.get(1) {
    Some(name) => greets(name),
    None => panic!("No name provided!"), //quit program
}

// if / let
if let Some(name) = args.get(1) {
    greets(name);
} else {
    eprintln!("No name provided");
}

// unwrap
// `unwrap` returns a `panic` when it receives a `None`.
let name = args.get(1).unwrap();

// expect
// panic like unwrap but with message
let name = args.get(1).expect("Name is required");
```

:bulb: More about flow of control :

- [Optional doc](https://doc.rust-lang.org/rust-by-example/std/option.html?highlight=option#option)
- [if/let flow](https://doc.rust-lang.org/rust-by-example/flow_control/if_let.html)
- [panic or not panic](https://doc.rust-lang.org/book/ch09-03-to-panic-or-not-to-panic.html)

## Retrieve application argument

Try to pass an integer as an argument and print square

```Bash
cargo run -- 2
```

What happens when we execute this code ?

```rust
let x: u32 = args.get(1).unwrap();
println!("{} square is {}", x , x*x );
```

<details>
<summary>&#128073; Check by yourself to see error</summary>

```bash
let x: u32 = args.get(1).unwrap();
  |                ---   ^^^^^^^^^^^^^^^^^^^^ expected `u32`, found `&String`
  |                |
  |                expected due to this
```

Arguments passed to app are Strings, you need to cast String to integer

</details>

### Parse application argument

Try to `parse` String into integer can fail\
We can either have a Succes either an error

String parse method returns `Result`:

```rust
enum Result<T, E> {
   Ok(T),
   Err(E),
}
```

Examples of usage

```rust
// Ok
let four: u32 = "4".parse().unwrap(); // explicit type annotation
let four = "4".parse::<u32>().unwrap(); // same but with turbofish annotation

//Err
let notaninteger = "whatever".parse::<u32>().unwrap();
let toomuch = "300".parse::<u8>().unwrap();
```

We can manage errors like we do with `Option`

Nb: `Err` provides a message about failure

```rust
match args.get(1).unwrap().parse::<u32>() {
    Ok(x) => println!("square of {} is {}", x, x * x),
    Err(e) => println!("error parsing : {}", e),
}
```

:bulb: Notes

- [Result](https://doc.rust-lang.org/std/result/)
- [Str parse method](https://doc.rust-lang.org/std/primitive.str.html#method.parse)

### Type conversion

When you want to match a type to another, Rust provides `trait` to implement custom behaviour.

#### From trait

Use can `From` trait when you assume the conversion has no errors to it.

```rust
use std::convert::From;

#[derive(Debug)]
struct Number {
    value: i32,
}

impl From<i32> for Number {
    fn from(item: i32) -> Self {
        Number { value: item }
    }
}

fn main() {
    let num = Number::from(30);
    println!("My number is {:?}", num);
}
```

> If you handle `From` traitn you don't have to handle equivalent `Into` trait.

#### TryFrom trait

Use `tryFrom` trait when conversion can fail (you handle this case)

It uses Result enum type

```rust
// example to convert an integer to a custom struct
struct GreaterThanZero(i32);

impl TryFrom<i32> for GreaterThanZero {
    type Error = &'static str;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if value <= 0 {
            Err("GreaterThanZero only accepts value superior than zero!")
        } else {
            Ok(GreaterThanZero(value))
        }
    }
}

// Returns an error because `big_number` is too big to
// fit in an `i32`.
let try_smaller_number = i32::try_from(big_number);
assert!(try_smaller_number.is_err());

// Returns `Ok(3)`.
let try_successful_smaller_number = i32::try_from(3);
assert!(try_successful_smaller_number.is_ok());
```

:pushpin: Remember

- You must handle conversion yourself with traits
- You decide if failure ends your program or not

:bulb: notes

- [TryFrom documentation](https://doc.rust-lang.org/std/convert/trait.TryFrom.html)
- [From / Into](https://doc.rust-lang.org/rust-by-example/conversion/from_into.html)
- [convert module documentation](https://doc.rust-lang.org/std/convert/index.html)

### :pencil: Exercise : Update your application to execute two commands

Greetings

```bash
cargo run greets You

Hello You !
```

Bench

```bash
cargo run 165 5 4

Joe can do 5 reps of 165 kilos on the bench press
Joe can do 3300 kilos on the bench press

```

Read the compiler errors 🤷🏻‍♂️

:bulb: Tips

- `String` can be created from `&str` with `String::from()`
- `&str` can be created from String with `.as_str()̀`
- Use type conversion to convert cli argument to Game `enum`

:books: More resources

- [String](https://doc.rust-lang.org/rust-by-example/std/str.html?highlight=String%3A%3Afrom#strings)

## :clap: Congrats

This ain't over !
