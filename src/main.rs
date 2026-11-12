mod dice;
mod selection;
mod game;
mod ai;

use game::GameState;
use ai::{ai_take_turn, AiTurnResult};
use std::io::{self, Write};

fn read_line() -> String {
    let mut line = String::new();
    io::stdin().read_line(&mut line).expect("Failed to read line");
    line
}

fn display_roll(rolled: &[u32]) {
    let values: Vec<String> = rolled.iter().map(|v| v.to_string()).collect();
    let positions: Vec<String> = (1..=rolled.len()).map(|i| i.to_string()).collect();
    println!("[ {} ]", values.join("  "));
    println!("  {}", positions.join("  "));
}

fn display_scores(state: &GameState) {
    println!("You: {}  |  AI: {}  |  Turn: {}", state.player_score, state.ai_score, state.turn_score);
}

fn read_positions(n_dice: usize) -> Vec<usize> {
    loop {
        print!("Select positions: ");
        io::stdout().flush().unwrap();
        let line = read_line();
        let parts: Vec<&str> = line.trim().split_whitespace().collect();
        if parts.is_empty() {
            println!("Error: enter at least one position.");
            continue;
        }
        let mut positions = Vec::new();
        let mut ok = true;
        for part in &parts {
            match part.parse::<usize>() {
                Ok(n) if n >= 1 && n <= n_dice => positions.push(n),
                Ok(n) => {
                    println!("Error: position {} out of range (1–{}).", n, n_dice);
                    ok = false;
                    break;
                }
                Err(_) => {
                    println!("Error: '{}' is not a valid position.", part);
                    ok = false;
                    break;
                }
            }
        }
        if ok {
            return positions;
        }
    }
}

fn read_bank_or_roll() -> char {
    loop {
        print!("Bank (b) or Roll again (r)? ");
        io::stdout().flush().unwrap();
        match read_line().trim() {
            "b" => return 'b',
            "r" => return 'r',
            _ => println!("Enter 'b' to bank or 'r' to roll again."),
        }
    }
}

// Returns true if the player won this turn.
fn player_take_turn(state: &mut GameState) -> bool {
    loop {
        let rolled = state.roll();

        if GameState::check_farkle(&rolled) {
            println!("Farkle! You lose your turn score.");
            state.reset_turn();
            return false;
        }

        display_roll(&rolled);
        display_scores(state);

        loop {
            let positions = read_positions(rolled.len());
            match state.select_positions(&rolled, &positions) {
                Ok(score) => {
                    println!("+{}. Turn total: {}.", score, state.turn_score);
                    break;
                }
                Err(e) => println!("Error: {}", e),
            }
        }

        match read_bank_or_roll() {
            'b' => {
                let (total, won) = state.bank();
                println!("You bank. Score: {}.", total);
                return won;
            }
            _ => { /* roll remaining dice */ }
        }
    }
}

fn main() {
    println!("=== Ferkle ===");
    println!("First to 2000 wins.\n");

    let mut state = GameState::new();

    loop {
        println!("--- Your turn ---");
        if player_take_turn(&mut state) {
            println!("You win! Final scores — You: {}, AI: {}.", state.player_score, state.ai_score);
            break;
        }

        println!("\n--- AI's turn ---");
        match ai_take_turn(&mut state) {
            AiTurnResult::Banked(_) if state.ai_score >= 2000 => {
                println!("AI wins. Final scores — You: {}, AI: {}.", state.player_score, state.ai_score);
                break;
            }
            _ => {}
        }
        println!();
    }
}
