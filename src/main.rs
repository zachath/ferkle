mod dice;
mod selection;

use dice::{Dice, OrdinaryDie, EvenDie, OddDie};
use selection::selection;

fn main() {
    let ordinary_die = OrdinaryDie;
    let even_die = EvenDie;
    let odd_die = OddDie;

    println!("OrdinaryDie: {}", ordinary_die.roll());
    println!("EvenDie: {}", even_die.roll());
    println!("OddDie: {}", odd_die.roll());
    
    match selection(&[1, 1, 1, 5, 5]) {
        Ok(score) => {
            println!("Selection:  {}", score);
        },
        Err(e) => {
            println!("Error in Selection:  {}", e);
        }
    }
}
