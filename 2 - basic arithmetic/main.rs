
/*
 * Iesus Hominum Salvator s2
 */

fn main() {
    println!("Operations: \n");
    operations();

    println!("\n\n\nSignals: \n");
    signals();

    println!("\n\n\nAddition: \n");
    addition();

    println!("\n\n\nMultiplication: \n");
    multiplication();

    println!("\n\n\nDivision: \n");
    division();
}

fn operations() {
    let a = |x,y| x + y;
    println!("\ta: {}", a(10, 20));

    let calc = |x,y,c| {
        match c {
            '+' => x + y,
            '-' => x - y,
            '*' => x * y,
            '/' => x / y,
            _ => 0.0,
        }
    };

    let b = 10.0;
    let c = 20.0;

    println!("\t+: {}", calc(b, c, '+'));
    println!("\t-: {}", calc(b, c, '-'));
    println!("\t*: {}", calc(b, c, '*'));
    println!("\t/: {}", calc(b, c, '/'));
}

fn signals() {
    let show = |a, b| println!("\t{a} = {b}");

    show(
        (4) + (-2) + (1) - (1) ,
         4     -2  +  1  -  1
    );

    show(
        (-6) - (10) + (-3) ,
         -6  -  10     -3
    );

    show(
        (10) + (-5) - (-10) ,
         10     -5  +   10
    );

    show(
        (15) + (-10) - (4) + ( 20 - 30) + (-1),
         15     -10  -  4  + (-10     )    -1
    );

    show(
        200 + (-23) - (-30) + 78 + (50 / 100) - 10,
        200    -23  +   30  + 78 + (50 / 100) - 10
    );
}

fn addition() {
    println!("\tCommutative: {} = {} = {}",
        10 + 20 + 30,
        30 + 20 + 10,
        20 + 10 + 30
    );

    println!("\tAssociative: {} = {} = {} = {} = {} = {}",
        (10 +  20) + 30,
        (30 +  20) + 10,
        (20 +  10) + 30,
         10 + (20  + 30),
         30 + (10  + 20),
         20 + (30  + 10),
    );
}

fn multiplication() {
    println!("\tCommutative: {} = {} = {}",
        10 * 20 * 30 * 40,
        40 * 30 * 20 * 10,
        30 * 10 * 40 * 20,
    );

    println!("\tAssociative: {} = {} = {}",
        10 *  20 * (30  * 40),
        40 *  30 * (20  * 10),
        30 * (20 *  40) * 10,
    );

    println!("\tDistributive: {} = {}",
         10 * (20 + 30),
        (10 * 20) + (10 * 30),
    );

    println!("\tNeutral element: {}, {}, {}, {}",
        10 * 1,
        20 * 1,
        1 * 10,
        1 * 20
    );

    println!("\tAnnulment: {}, {}, {}, {}\n",
        10 * 0,
        20 * 0,
        0 * 10,
        0 * 20
    );
    
    let mult = |a, b| println!("\t{} * {} = {}", a, b, a * b);

    mult(0.2, 0.3);
    mult(1.2, 0.4);
    mult(-12.0, 5.0);
    mult(5.0, -12.0);
    mult(15.0, 0.02);
    mult(15.0, 0.2);
    mult(0.2, 0.3);
    mult(2.0 / 10.0, 3.0 / 10.0);
    mult(1.2, 0.4);
    mult(12.0 / 10.0, 4.0  / 10.0);
    mult(29.01, 4.0);
    mult(2901.0 / 100.0, 40.0   / 10.0);
}

fn division() {
    let quotient = 4.0;
    let divider = 2.0;
    let rest = 1.0;
    let dividend = divider * quotient + rest;

    println!("\t{} = {} x {} + {}", dividend, divider, quotient, rest);
    println!("\t{} / {} = {}", dividend, divider, dividend / divider);

    let mut a = 433.0;
    let mut b = 6.0;
    println!("\t{} / {} = {}", a, b, a / b);

    a = 8.0;
    b = 1230.0;
    println!("\t{} / {} = {}", a, b, a / b);
}


