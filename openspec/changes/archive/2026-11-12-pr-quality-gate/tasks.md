# Tasks

## 1. Fix existing warnings

- [x] 1.1 Add `#[allow(dead_code)]` to `EvenDie` and `OddDie` in `src/dice.rs`; verify `cargo clippy -- -D warnings` emits no warnings for those structs
- [x] 1.2 Change `AiTurnResult::Banked(u32)` to `Banked(())` in `src/ai.rs` and update all match sites in `src/main.rs`; verify `cargo clippy -- -D warnings` passes with zero warnings across the whole crate

## 2. GitHub Actions workflow

- [x] 2.1 Create `.github/workflows/ci.yml` with a `ci` job triggered on `pull_request` to `master`, using `ubuntu-latest`, checking out the repo, installing the stable Rust toolchain (with `rustfmt` and `clippy` components) via `dtolnay/rust-toolchain@stable`, and enabling the `Swatinem/rust-cache` action; verify the file is valid YAML and the job structure is correct
- [x] 2.2 Add a `cargo fmt -- --check` step to the job; verify by temporarily introducing a formatting violation locally and confirming the command exits non-zero
- [x] 2.3 Add a `cargo clippy -- -D warnings` step to the job; verify it passes cleanly against the fixed codebase
- [x] 2.4 Add a `cargo test` step to the job; verify all 18 tests pass
- [x] 2.5 Add `taiki-e/install-action@v2` to install `cargo-audit`, then add a `cargo audit` step; verify `cargo audit` exits zero against the current `Cargo.lock`
