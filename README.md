# rust-playground
learn rust and do something funny

How to add and run separate independent binaries
- Place each binary file under src/bin with a filename like src/bin/aaa.rs or src/bin/bbb.rs.
- When using cargo you refer to the binary by its name only (no ".rs").

Examples:
- Build a specific binary:
  cargo build --bin aaa
  cargo build --bin bbb

- Run a specific binary:
  cargo run --bin aaa
  cargo run --bin bbb

Notes
- The binary name is the filename without the .rs extension (for example src/bin/aaa.rs -> binary name `aaa`).
- You can keep multiple binaries under src/bin; Cargo will discover them automatically.
