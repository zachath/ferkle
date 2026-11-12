use rand::RngExt;

pub trait Dice {
    fn roll(&self) -> u32;
}

// https://kingdom-come-deliverance.fandom.com/wiki/Ordinary_die
pub struct OrdinaryDie;

impl Dice for OrdinaryDie {
    fn roll(&self) -> u32 {
        rand::rng().random_range(1..=6)
    }
}

//https://kingdom-come-deliverance.fandom.com/wiki/Even_die
#[allow(dead_code)]
pub struct EvenDie;

impl Dice for EvenDie {
    fn roll(&self) -> u32 {
        let mut rng = rand::rng();

        if rng.random_range(1..=10) <= 8 {
            [2, 4, 6][rng.random_range(0..3)]
        } else {
            [1, 3, 5][rng.random_range(0..3)]
        }
    }
}

// https://kingdom-come-deliverance.fandom.com/wiki/Odd_die/KCD2
#[allow(dead_code)]
pub struct OddDie;

impl Dice for OddDie {
    fn roll(&self) -> u32 {
        let mut rng = rand::rng();

        if rng.random_range(1..=10) <= 8 {
            [1, 3, 5][rng.random_range(0..3)]
        } else {
            [2, 4, 6][rng.random_range(0..3)]
        }
    }
}
