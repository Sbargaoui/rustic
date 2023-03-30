@default:
    just --list

# format and lint scripts
@fmt:
    #!/usr/bin/env bash
    cargo fmt && cargo clippy 

@install:
    #!/usr/bin/env bash
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
