use std::collections::HashSet;
use crate::dice::{Dice, OrdinaryDie};
use crate::selection::selection;

pub struct GameState {
    pub player_score: u32,
    pub ai_score: u32,
    pub turn_score: u32,
    pub dice_remaining: u8,
}

impl GameState {
    pub fn new() -> Self {
        GameState {
            player_score: 0,
            ai_score: 0,
            turn_score: 0,
            dice_remaining: 6,
        }
    }

    pub fn roll(&self) -> Vec<u32> {
        let die = OrdinaryDie;
        (0..self.dice_remaining).map(|_| die.roll()).collect()
    }

    // positions are 1-based indices into rolled
    pub fn select_positions(&mut self, rolled: &[u32], positions: &[usize]) -> Result<u32, String> {
        if positions.is_empty() {
            return Err("Select at least one die.".to_string());
        }
        let mut seen = HashSet::new();
        for &pos in positions {
            if !seen.insert(pos) {
                return Err(format!("Duplicate position: {}.", pos));
            }
            if pos == 0 || pos > rolled.len() {
                return Err(format!("Position {} is out of range (1–{}).", pos, rolled.len()));
            }
        }
        let faces: Vec<u32> = positions.iter().map(|&p| rolled[p - 1]).collect();
        let score = selection(&faces)?;
        self.dice_remaining -= positions.len() as u8;
        self.turn_score += score;
        if self.dice_remaining == 0 {
            self.dice_remaining = 6;
        }
        Ok(score)
    }

    pub fn check_farkle(rolled: &[u32]) -> bool {
        let n = rolled.len();
        for mask in 1u32..(1u32 << n) {
            let faces: Vec<u32> = (0..n)
                .filter(|&i| mask & (1 << i) != 0)
                .map(|i| rolled[i])
                .collect();
            if selection(&faces).is_ok() {
                return false;
            }
        }
        true
    }

    // Bank for the player. Returns (new_player_score, player_won).
    pub fn bank(&mut self) -> (u32, bool) {
        self.player_score += self.turn_score;
        self.turn_score = 0;
        self.dice_remaining = 6;
        (self.player_score, self.player_score >= 2000)
    }

    // Bank for the AI. Returns (new_ai_score, ai_won).
    pub fn bank_ai(&mut self) -> (u32, bool) {
        self.ai_score += self.turn_score;
        self.turn_score = 0;
        self.dice_remaining = 6;
        (self.ai_score, self.ai_score >= 2000)
    }

    pub fn reset_turn(&mut self) {
        self.turn_score = 0;
        self.dice_remaining = 6;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_state_defaults() {
        let s = GameState::new();
        assert_eq!(s.player_score, 0);
        assert_eq!(s.ai_score, 0);
        assert_eq!(s.turn_score, 0);
        assert_eq!(s.dice_remaining, 6);
    }

    #[test]
    fn roll_count_and_range() {
        let s = GameState::new();
        let rolled = s.roll();
        assert_eq!(rolled.len(), 6);
        assert!(rolled.iter().all(|&v| (1..=6).contains(&v)));
    }

    #[test]
    fn select_valid_single() {
        let mut s = GameState::new();
        let rolled = vec![1, 2, 3, 4, 5, 6];
        let score = s.select_positions(&rolled, &[1]).unwrap();
        assert_eq!(score, 100);
        assert_eq!(s.turn_score, 100);
        assert_eq!(s.dice_remaining, 5);
    }

    #[test]
    fn select_valid_triple() {
        let mut s = GameState::new();
        let rolled = vec![3, 3, 3, 2, 4, 6];
        let score = s.select_positions(&rolled, &[1, 2, 3]).unwrap();
        assert_eq!(score, 300);
        assert_eq!(s.turn_score, 300);
        assert_eq!(s.dice_remaining, 3);
    }

    #[test]
    fn select_invalid_non_scoring() {
        let mut s = GameState::new();
        let rolled = vec![2, 3, 4, 2, 3, 4];
        assert!(s.select_positions(&rolled, &[1]).is_err());
    }

    #[test]
    fn select_out_of_range() {
        let mut s = GameState::new();
        let rolled = vec![1, 2, 3];
        assert!(s.select_positions(&rolled, &[4]).is_err());
        assert!(s.select_positions(&rolled, &[0]).is_err());
    }

    #[test]
    fn select_hot_dice_reset() {
        let mut s = GameState { dice_remaining: 3, ..GameState::new() };
        let rolled = vec![1, 1, 1];
        s.select_positions(&rolled, &[1, 2, 3]).unwrap();
        assert_eq!(s.dice_remaining, 6);
    }

    #[test]
    fn check_farkle_true() {
        assert!(GameState::check_farkle(&[2, 2, 3, 3, 4, 6]));
        assert!(GameState::check_farkle(&[2, 3, 4, 6, 2, 3]));
    }

    #[test]
    fn check_farkle_false() {
        assert!(!GameState::check_farkle(&[1, 2, 3, 4, 5, 6]));
        assert!(!GameState::check_farkle(&[2, 3, 4, 5, 6, 2]));
    }

    #[test]
    fn bank_accumulates_and_resets() {
        let mut s = GameState::new();
        s.player_score = 1800;
        s.turn_score = 300;
        let (total, won) = s.bank();
        assert_eq!(total, 2100);
        assert!(won);
        assert_eq!(s.turn_score, 0);
        assert_eq!(s.dice_remaining, 6);
    }
}
