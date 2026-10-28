mod dice;

use dice::{Dice, SixSidedDie, EvenDie, OddDie};

fn main() {
    let six_sided_die = SixSidedDie;
    let even_die = EvenDie;
    let odd_die = OddDie;

    println!("SixSidedDie: {}", six_sided_die.roll());
    println!("EvenDie: {}", even_die.roll());
    println!("OddDie: {}", odd_die.roll());
}
