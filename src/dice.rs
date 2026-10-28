use rand::{RngExt};

pub trait Dice {
    fn roll(&self) -> u32;
}

pub struct SixSidedDie;

impl Dice for SixSidedDie {
    fn roll(&self) -> u32 {
        return rand::rng().random_range(1..=6);
    }
}

//https://kingdom-come-deliverance.fandom.com/wiki/Even_die
pub struct EvenDie;

impl Dice for EvenDie {
    fn roll(&self) -> u32 {
        let mut rng = rand::rng();
        
        if rng.random_range(1..=10) <= 8 {
            return [2, 4, 6][rng.random_range(0..3)]
        } else {
            return [1, 3, 5][rng.random_range(0..3)]
        }
    }
}

// https://kingdom-come-deliverance.fandom.com/wiki/Odd_die/KCD2
pub struct OddDie;

impl Dice for OddDie {
    fn roll(&self) -> u32 {
        let mut rng = rand::rng();
        
        if rng.random_range(1..=10) <= 8 {
            return [1, 3, 5][rng.random_range(0..3)]
        } else {
            return [2, 4, 6][rng.random_range(0..3)]
        }
    }
}