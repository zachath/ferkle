# Proposal

## Why

The game's scoring logic is fully implemented but `main.rs` is a scratch pad — there is no way to actually play Farkle. This change wires the existing logic into a playable 1v1 game against a CPU opponent driven entirely from the terminal.

## What Changes

- Replace the scratch-pad `main.rs` with an interactive game loop
- Add a `game` module: `GameState`, turn execution, Farkle detection, hot-dice reset, and win condition
- Add an `ai` module: best-subset finder (brute-force 2^6 subsets) and bank/roll decision logic
- CLI input: player selects dice by position (1–6); output is terse and informational

## Capabilities

### New Capabilities

- `game`: Turn-based game state — rolling, position-based selection, Farkle detection, hot-dice reset, banking, and first-to-2000 win condition
- `ai-opponent`: CPU opponent that automatically picks the highest-scoring subset of its roll and banks when its accumulated turn score meets a threshold or dice count is low
- `cli`: Interactive terminal interface — displays dice with position labels, prompts for position-based selection, prints terse per-action feedback, and declares the winner

### Modified Capabilities

## Impact

- `src/main.rs`: full rewrite — becomes the CLI entry point and game orchestrator
- `src/game.rs`: new file — `GameState` struct, turn loop, Farkle detection, hot-dice logic
- `src/ai.rs`: new file — best-subset selection, bank/roll decision
- No new crate dependencies (rand already present)
