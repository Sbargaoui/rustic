# Learn how organize code

Our cli has two functionalities (greetings and bench) until now.\
It is always better to split the desired logic in different files.

## :dart: Objectives

- use modules

## Organize code in modules

Modules and Let help you control the architecture, scope, and privacy of paths

Let's split our code in differents files

```bash
|-src
  |- main.rs
  |- greetings.rs
```

```rust
// greetings.rs
pub fn greets(name: &str) {
    println!("Hello, {} 🦀 !", name);
}
```

Nb: functions are private by defaut, you must add `pub` keyword

```rust
// main.rs
use std::env;

mod greetings; //declare module
use greetings::greets; // import function

fn main() {
    let args: Vec<String> = env::args().collect();

    let name = args.get(1).expect("Name is required");
    greets(&name);
}
```

:pushpin: Remember

- declare module and declare usage
- visibibility is private by default

:books: More resources

- [crates and modules](https://doc.rust-lang.org/book/ch07-00-managing-growing-projects-with-packages-crates-and-modules.html)

## :pencil: Exercice: Split app in different modules

Expected tree

```bash
|-src
  |- main.rs
  |- greetings.rs
  |- bench.rs
```

:bulb: Tips

- do not forget the default visiblity
