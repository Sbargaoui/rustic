# Environement Setup

## :dart: Objectives

Make your Rust setup environement ready

- Initialize your project
- Compile and run

## :pencil: Setup project with cargo

[Cargo](https://doc.rust-lang.org/cargo/) is the Rust Package Manager \
It manages dependencies, tasks, project creation, workspaces... (can be compare to npm/yarn)

### Create a new application

```bash
cd /your/project/directory/
cargo new rustic
```

Move to your application directory :

```bash
cd rustic
```

> :bulb: Notes
>
> Cargo creates a git repository with all the structure by default  
> use `cargo new --help` for more options

### How to build an application ?

Launch compilation with `cargo` command:

```bash
cargo build
```

Look at the newly created directories and files:

```bash
ls ./target/
```

A new `debug` directory has been created in `target`

You can execute the built file with:

```bash
./target/debug/rustic
```

Launch compilation with release profile:

```bash
cargo build --release
```

You have a new `release` directory in `./target`

```bash
ls ./target
.
..
debug
release
```

### Compile and Run

A common way to run code when developing is to use `cargo run`

```bash
cargo run

   Finished dev [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/rustic-init`
Hello, world !
```

cargo compiles with the debug profile if needed and runs compiled programs

### How to test ?

Launch `cargo`:

```bash
cargo test

    Finished test [unoptimized + debuginfo] target(s) in 0.01s
     Running unittests src/main.rs

running 0 tests
```

:exclamation: 🛑 No tests are written yet...

## :pencil: Problem to solve

Change the program output with :

```bash
Hello w💩rld !
```

> :bulb: Notes
>
> - emojis are not ASCII Chars
> - Strings are stored as Utf8 encoded references in Rust

## :clap: Congrats

You have a working Rust setup !

Check the expected source code [here](./solution/src/main.rs)

## :memo: Summary

What you have learned

- How to intialize a project with cargo
- Build project
- Run project
- Test project

## :books: Additional resources

- [Cargo profiles](https://doc.rust-lang.org/book/ch14-01-release-profiles.html)
- [Strings](https://doc.rust-lang.org/rust-by-example/std/str.html)
- [Rust string and UTF8](https://doc.rust-lang.org/book/ch08-02-strings.html)

### Next Part

TBD
