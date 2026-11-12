use crate::game::GameState;
use crate::selection::selection;

pub enum AiTurnResult {
    Banked(()),
    Farkled,
}

// Returns the 1-based positions and total score of the highest-scoring valid subset,
// or None if no subset scores (Farkle).
pub fn best_subset(rolled: &[u32]) -> Option<(Vec<usize>, u32)> {
    let n = rolled.len();
    let mut best: Option<(Vec<usize>, u32)> = None;

    for mask in 1u32..(1u32 << n) {
        let positions: Vec<usize> = (0..n)
            .filter(|&i| mask & (1 << i) != 0)
            .map(|i| i + 1)
            .collect();
        let faces: Vec<u32> = positions.iter().map(|&p| rolled[p - 1]).collect();
        if let Ok(score) = selection(&faces) {
            let better = best
                .as_ref()
                .is_none_or(|(_, best_score)| score > *best_score);
            if better {
                best = Some((positions, score));
            }
        }
    }
    best
}

pub fn should_bank(turn_score: u32, dice_remaining: u8) -> bool {
    turn_score >= 350 || dice_remaining <= 2
}

pub fn ai_take_turn(state: &mut GameState) -> AiTurnResult {
    loop {
        let rolled = state.roll();

        if GameState::check_farkle(&rolled) {
            println!("AI: Farkle!");
            state.reset_turn();
            return AiTurnResult::Farkled;
        }

        let (positions, score) = best_subset(&rolled).unwrap();
        let values: Vec<u32> = positions.iter().map(|&p| rolled[p - 1]).collect();
        state.select_positions(&rolled, &positions).unwrap();

        if should_bank(state.turn_score, state.dice_remaining) {
            let new_total = state.ai_score + state.turn_score;
            println!("AI banks. Score: {}.", new_total);
            state.bank_ai();
            return AiTurnResult::Banked(());
        } else {
            println!(
                "AI keeps {:?} (+{}), rolls {} dice.",
                values, score, state.dice_remaining
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn best_subset_finds_highest() {
        // [1, 5, 5, 2, 3, 4]: full 6-die set = 1-5 straight (500) + spare 5 (50) = 550
        let rolled = vec![1, 5, 5, 2, 3, 4];
        let (_, score) = best_subset(&rolled).unwrap();
        assert_eq!(score, 550);
    }

    #[test]
    fn best_subset_none_on_farkle() {
        let rolled = vec![2, 3, 4, 2, 3, 4];
        assert!(best_subset(&rolled).is_none());
    }

    #[test]
    fn should_bank_threshold_boundary() {
        assert!(!should_bank(349, 6));
        assert!(should_bank(350, 6));
    }

    #[test]
    fn should_bank_low_dice_boundary() {
        assert!(!should_bank(0, 3));
        assert!(should_bank(0, 2));
    }
}
