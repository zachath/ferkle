# Tasks

## 1. Game state module (game.rs)

- [x] 1.1 Create `src/game.rs` with a `GameState` struct holding `player_score`, `ai_score`, `turn_score`, and `dice_remaining`; add `mod game` to `main.rs`; verify `cargo check` passes
- [x] 1.2 Implement `GameState::new()` returning default-zeroed state with `dice_remaining = 6`; verify via unit test that a fresh `GameState` has all zeros and 6 dice
- [x] 1.3 Implement `GameState::roll(&self) -> Vec<u32>` that rolls `dice_remaining` `OrdinaryDie`s and returns their face values; verify it always returns exactly `dice_remaining` values each in 1–6
- [x] 1.4 Implement `GameState::select_positions(&mut self, rolled: &[u32], positions: &[usize]) -> Result<u32, String>` that maps 1-based positions to face values, calls `selection::selection()`, on success subtracts `positions.len()` from `dice_remaining` and adds the score to `turn_score`, and on hot-dice (dice_remaining == 0) resets `dice_remaining` to 6; verify unit tests cover: valid single/triple selection, invalid non-scoring selection, out-of-range position, hot-dice reset
- [x] 1.5 Implement `GameState::check_farkle(rolled: &[u32]) -> bool` that returns true when no non-empty subset of `rolled` passes `selection()`; verify unit tests: a roll of all 2s/3s/4s/6s returns true, a roll containing a 1 or 5 returns false
- [x] 1.6 Implement `GameState::bank(&mut self) -> (u32, bool)` that adds `turn_score` to `player_score`, resets `turn_score = 0` and `dice_remaining = 6`, and returns `(new_player_score, player_score >= 2000)`; verify unit test: banking 1800 + 300 = 2100 returns `(2100, true)`

## 2. AI module (ai.rs)

- [x] 2.1 Create `src/ai.rs` with `best_subset(rolled: &[u32]) -> Option<(Vec<usize>, u32)>` that iterates all 63 non-empty subsets of `rolled`, calls `selection()` on each, and returns the 1-based positions and score of the highest-scoring valid subset, or `None` if none score; add `mod ai` to `main.rs`; verify `cargo check` passes
- [x] 2.2 Implement `should_bank(turn_score: u32, dice_remaining: u8) -> bool` returning true when `turn_score >= 350` or `dice_remaining <= 2`; verify unit tests cover threshold boundary (349 false, 350 true) and low-dice boundary (3 false, 2 true)
- [x] 2.3 Implement `ai_take_turn(state: &mut GameState) -> AiTurnResult` that loops: rolls, Farkle-checks, selects the best subset, prints the terse narration line ("AI keeps [<values>] (+<points>), rolls <n> dice." or "Farkle!" or "AI banks. Score: <total>."), calls bank on the state's AI side when `should_bank` is true, and returns the result; verify: a forced all-Farkle sequence forfeits turn score, a sequence that crosses 350 triggers a bank

## 3. Interactive CLI (main.rs rewrite)

- [x] 3.1 Write helper `display_roll(rolled: &[u32])` that prints the face values and 1-based position labels in the two-line format from the spec; verify by inspection that the output matches the spec format exactly
- [x] 3.2 Write helper `display_scores(state: &GameState)` that prints player total, AI total, and accumulated turn score on one or two lines; verify by inspection
- [x] 3.3 Implement `read_positions(n_dice: usize) -> Vec<usize>` that reads a line from stdin, parses space-separated integers, rejects out-of-range or non-integer input with an error line, and re-prompts until valid input is received; verify: "1 3 5" parses to [1,3,5], "0 7 abc" triggers re-prompt
- [x] 3.4 Implement the player turn loop: roll, Farkle-check (print "Farkle!" and return), display roll + scores, prompt for positions, call `select_positions`, then loop on "Bank (b) or Roll again (r)?" until the player banks or Farkles; verify that an end-to-end player turn compiles and exits cleanly
- [x] 3.5 Replace `main()` with the full game loop: alternate player turn / AI turn, check win condition after each bank, print winner announcement and exit; verify `cargo run` starts, plays through at least one complete game to a winner, and prints the correct winner line
