# Proposal

## Why

The codebase currently has no automated quality gate — `cargo check` already emits three warnings, and there is nothing stopping a PR from introducing more regressions, format drift, or new lint failures. Adding a CI workflow on pull requests enforces quality automatically and keeps the warning count at zero.

## What Changes

- Add `.github/workflows/ci.yml` running four checks on every pull request to `master`: format check, Clippy (warnings-as-errors), tests, and dependency audit
- Fix the three existing `cargo check` warnings so the new gate passes from day one:
  - `src/dice.rs`: annotate `EvenDie` and `OddDie` with `#[allow(dead_code)]` — these are intentional KCD2 dice variants not yet wired into gameplay
  - `src/ai.rs`: drop the unused `u32` payload from `AiTurnResult::Banked`

## Capabilities

### New Capabilities

- `ci`: GitHub Actions workflow that runs `cargo fmt --check`, `cargo clippy -- -D warnings`, `cargo test`, and `cargo audit` on every pull request targeting `master`

### Modified Capabilities

## Impact

- `.github/workflows/ci.yml`: new file
- `src/dice.rs`: `#[allow(dead_code)]` on `EvenDie` and `OddDie`
- `src/ai.rs`: `Banked(u32)` → `Banked(())`, update all match sites
- No new runtime dependencies; `cargo-audit` runs as a CI install step
