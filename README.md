# rust-kata ⚙️

## :pencil: Requirements

You'll need to install:

- [Rust installation instructions](https://www.rust-lang.org/tools/install)

TLDR;

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

If you already have `just` installed :

```bash
just install
```

### Verify your toolchain version

Minimum Version : 1.69+ (tested)

```bash
rustc --version
```

```bash
rustc 1.68.0 (2c8cc3432 2023-03-06)
```

Keep your rust up-to-date with the latest release of rust, type:

```bash
rustup update
```

### Choose your IDE

- VS Code:
  - [Rust Analyzer Language Server Protocol](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)
  - [TOML](https://marketplace.visualstudio.com/items?itemName=bungcip.better-toml)
- Jetbrains Rust:
  - https://www.jetbrains.com/fr-fr/rust/
- Vim plugin :
  - https://github.com/simrat39/rust-tools.nvim

[More tools on Rust website](https://www.rust-lang.org/tools)

### Alternative to IDE

> For the laziest ones out here

- Online Playground
  - https://play.rust-lang.org/
- gitpod (inside brower or in your IDE)
  - [![Open in Gitpod](https://gitpod.io/button/open-in-gitpod.svg)](https://gitpod.io/#https://github.com/Sbargaoui/rustic)

### :pencil: Run solutions :

```bash
// from git root directory
cargo run --bin init
cargo run --bin bench
cargo run --bin args
```

### :zap: Tests

> “Any fool can write code that a computer can understand. Good programmers write code that humans can understand.”

```bash
// all
cargo test

```

```bash
// all
cargo test

//specific
cargo test --bin init
cargo test --bin bench
cargo test --bin args
```

### 🚀 Get through this kata :

Follow these steps to build your first Rust cli :

```
- init
- bench
- modules
- args
- cli
```
