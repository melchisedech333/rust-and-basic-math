
use utils::*;

fn main() {
    let value = 4;
    println!("\nInput: {value}\n");

    for number in 2..=9 {
        println!("{}: {}", number, divisible_by(value, number));
    }
}


