
/*
 * Iesus Hominum Salvator
 */

use utils::*;

fn main() {
    let support = [ 2, 3, 4, 5, 6, 7, 8, 9, 10 ];
    let value   =   2160;

    for number in support {
        println!("{} is divisible by {}: {}", 
            value, number, divisible_by(value, number));
    }
}


