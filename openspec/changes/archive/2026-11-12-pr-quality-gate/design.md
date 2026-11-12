# Design

## Context

The project is a Rust binary hosted on GitHub (`zachath/ferkle`). There is no existing CI. The single external dependency is `rand`. Three `cargo check` warnings exist today and must be resolved before the gate is introduced, otherwise the first PR would immediately fail.

## Goals / Non-Goals

**Goals:**
- Zero-warning baseline on `master` before the gate lands
- Four-step gate (fmt, clippy, test, audit) that blocks any PR introducing a regression

**Non-Goals:**
- Release automation or deployment
- Cross-platform matrix (Linux runner is sufficient for a CLI game)
- Code coverage reporting

## Decisions

### Rust toolchain: stable, no pinned version

Using `actions/checkout` + the default `stable` Rust toolchain (via `dtolnay/rust-toolchain@stable`) keeps the project on the latest stable release automatically. Pinning a specific version (e.g. `1.87`) adds maintenance overhead with no benefit for a project this size.

The toolchain action installs `rustfmt` and `clippy` components alongside the compiler, so no separate install steps are needed for those.

### cargo-audit: install via cargo-install-action

`cargo audit` is not part of the standard toolchain. Options:

| Approach | Pro | Con |
|----------|-----|-----|
| `cargo install cargo-audit` | Simple | Re-compiles on every run (~60s) |
| `taiki-e/install-action@cargo-audit` | Cached binary, fast | Extra action dependency |
| `rustsec/audit-check` | Purpose-built | Opinionated output format |

Recommend `taiki-e/install-action@cargo-audit` — it downloads a prebuilt binary from GitHub Releases, avoiding the compile cost, with no meaningful downside for a public repo.

### Cargo artifact cache: enabled

Using `Swatinem/rust-cache` caches compiled dependencies across runs. With only `rand` as a dependency the benefit is small today but costs nothing and pays off as the dep tree grows.

### Step order: fmt → clippy → test → audit

Fail-fast ordering: format and lint failures are cheapest to diagnose and fix, so they run first. Tests run after to surface logic errors. Audit runs last since it requires a network fetch of the advisory DB and is least likely to fail on day-to-day development.

### Code fixes: annotation over suppression

`EvenDie` and `OddDie` are intentional future types. `#[allow(dead_code)]` placed on each struct documents that choice in the source rather than globally suppressing the lint or deleting the types. `Banked(u32)` → `Banked(())` removes the genuinely unused payload; the u32 was never read at any call site.

## Risks / Trade-offs

- **Advisory DB fetch can flake** — `cargo audit` fetches the RustSec DB on each run; a transient network error would fail the step. Acceptable: `cargo audit` has retry logic and flakes are rare on GitHub-hosted runners.
- **Stable toolchain drift** — a future stable release could introduce new Clippy lints that break the gate. Mitigation: `-D warnings` only fails on lints the current toolchain emits, so new lints only appear after a toolchain upgrade. Address them when they appear.
