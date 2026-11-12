# Design

## Context

The existing codebase has three pieces relevant to this change:

- `dice.rs` — `Dice` trait + `OrdinaryDie`, `EvenDie`, `OddDie`; each exposes `.roll() -> u32`
- `selection.rs` — `selection(&[u32]) -> Result<u32, String>`; validates that every selected die participates in a scoring combo and returns the total score
- `main.rs` — scratch pad; will be fully replaced

No persistence, no networking, no new crate dependencies needed.

## Goals / Non-Goals

**Goals:**
- Playable 1v1 turn loop in the terminal (player vs CPU)
- Position-based dice selection feeding the existing `selection()` validator
- Terse, readable output throughout
- CPU plays automatically with a simple greedy strategy

**Non-Goals:**
- Wager / money system
- Multiple difficulty levels
- Save/load
- Custom dice types at runtime (OrdinaryDie used for all dice)

## Decisions

### Module layout: game.rs + ai.rs + main.rs

Three modules keep concerns separate without over-engineering.

| Module | Responsibility |
|--------|---------------|
| `game.rs` | `GameState` struct, roll, validate selection, bank, Farkle/hot-dice, win check |
| `ai.rs` | Best-subset finder, bank/roll decision |
| `main.rs` | Read stdin, display output, orchestrate turns |

Alternative considered: single `main.rs`. Rejected because mixing display, input, game logic, and AI logic into one file makes the code hard to follow at ~200+ lines.

### AI subset finder: brute-force over all non-empty subsets

With at most 6 dice, there are 2^6 − 1 = 63 non-empty subsets. For each, extract the face values at those positions, call `selection()`, and track the highest-scoring valid result. O(63) calls to `selection()` is negligible.

Alternative considered: a bespoke greedy algorithm. Rejected because it duplicates scoring logic already in `selection()` and risks subtle divergence.

### Turn loop ownership: game.rs owns state, main.rs drives it

`GameState` exposes methods (`roll`, `select(positions)`, `bank`) that return typed results (enum or struct). `main.rs` calls these methods and handles all display and input. This keeps `game.rs` free of I/O and independently testable.

### Dice type: OrdinaryDie for all dice

The existing `OrdinaryDie` rolls 1–6 uniformly. `EvenDie` and `OddDie` are KCD2 special dice used by NPCs, not standard dice. The base game uses ordinary dice; special dice can be introduced later without touching the game loop.

### Turn score structure

`GameState` holds:
```
dice_remaining: u8      // dice available to roll this turn (starts 6, resets to 6 on hot dice)
turn_score: u32         // accumulated this turn, lost on Farkle
player_score: u32
ai_score: u32
```

After `select(positions)`: `dice_remaining -= positions.len()`, `turn_score += combo_score`.  
If `dice_remaining == 0`: hot dice — reset to 6.  
On `bank`: `player_score += turn_score`, reset `turn_score = 0`, `dice_remaining = 6`.

## Risks / Trade-offs

- **AI is predictable** — greedy always-max strategy is beatable once the player knows it. Acceptable for an initial implementation; threshold and strategy can be tuned later without touching specs.
- **No input buffering / raw mode** — input is line-buffered via `stdin().read_line()`. Simple and portable; no crossterm/ratatui dependency needed.
- **Farkle on first roll only** — Farkle is checked on the fresh roll, not after the player selects. A player who selects non-scoring dice gets an error and re-prompts; they can't voluntarily Farkle themselves.
