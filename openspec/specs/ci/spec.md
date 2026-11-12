# ci Specification

## Purpose

Automated quality gate that runs on every pull request to `master`, ensuring code is correctly formatted, lint-free, test-passing, and free of known vulnerable dependencies before merge.

## Requirements

### Requirement: CI runs on every pull request to master
The system SHALL trigger the CI workflow automatically on any pull request targeting the `master` branch.

#### Scenario: Pull request opened or updated
- **WHEN** a pull request targeting `master` is opened or a new commit is pushed to it
- **THEN** the CI workflow runs all four checks

### Requirement: Format check blocks merge on drift
The system SHALL run `cargo fmt --check` and fail the workflow if any file differs from canonical rustfmt output.

#### Scenario: Unformatted code in PR
- **WHEN** a PR contains a Rust source file that does not match `rustfmt` output
- **THEN** the format-check step fails and the PR is blocked

#### Scenario: All files correctly formatted
- **WHEN** all Rust source files match `rustfmt` output
- **THEN** the format-check step passes

### Requirement: Clippy runs with warnings-as-errors
The system SHALL run `cargo clippy -- -D warnings` and fail the workflow if any lint warning is emitted.

#### Scenario: PR introduces a new Clippy warning
- **WHEN** a PR introduces code that triggers any Clippy lint
- **THEN** the Clippy step fails and the PR is blocked

#### Scenario: No Clippy warnings
- **WHEN** the codebase emits no Clippy warnings
- **THEN** the Clippy step passes

### Requirement: Test suite must pass
The system SHALL run `cargo test` and fail the workflow if any test fails.

#### Scenario: PR breaks a test
- **WHEN** a PR causes one or more tests to fail
- **THEN** the test step fails and the PR is blocked

#### Scenario: All tests pass
- **WHEN** all tests pass
- **THEN** the test step passes

### Requirement: Dependency audit must pass
The system SHALL run `cargo audit` and fail the workflow if any dependency has a known security advisory.

#### Scenario: Vulnerable dependency detected
- **WHEN** a dependency version matches a RustSec advisory
- **THEN** the audit step fails and the PR is blocked

#### Scenario: No known vulnerabilities
- **WHEN** no dependency matches any RustSec advisory
- **THEN** the audit step passes
