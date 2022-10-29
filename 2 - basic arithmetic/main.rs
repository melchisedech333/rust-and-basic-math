
/*
 * Iesus Hominum Salvator
 */

mod general;
use general::*;

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

    println!("\tAnnulment: {}, {}, {}, {}",
        10 * 0,
        20 * 0,
        0 * 10,
        0 * 20
    );

    let mut a = 0.2;
    let mut b = 0.3;

    println!("\n\t{} * {} = {}", a, b, a * b);

    a = 1.2;
    b = 0.4;
    println!("\t{} * {} = {}", a, b, a * b);

    a = -12.0;
    b = 5.0;
    println!("\t{} * {} = {}", a, b, a * b);

    a = 5.0;
    b = -12.0;
    println!("\t{} * {} = {}", a, b, a * b);

    a = 15.0;
    b = 0.02;
    println!("\t{} * {} = {}", a, b, a * b);

    a = 15.0;
    b = 0.2;
    println!("\t{} * {} = {}", a, b, a * b);

    a = 0.2;
    b = 0.3;
    println!("\t{} * {} = {}", a, b, a * b);

    a = 2.0 / 10.0;
    b = 3.0 / 10.0;
    println!("\t{} * {} = {}", a, b, a * b);

    a = 1.2;
    b = 0.4;
    println!("\t{} * {} = {}", a, b, a * b);

    a = 12.0 / 10.0;
    b = 4.0  / 10.0;
    println!("\t{} * {} = {}", a, b, a * b);

    a = 29.01;
    b = 4.0;
    println!("\t{} * {} = {}", a, b, a * b);

    a = 2901.0 / 100.0;
    b = 40.0   / 10.0;
    println!("\t{} * {} = {}", a, b, a * b);
}

fn division() {
    let quotient = 4.0;
    let divider = 2.0;
    let rest = 1.0;
    let dividend = divider * quotient + rest;

    println!("{} = {} x {} + {}", dividend, divider, quotient, rest);
    println!("{} / {} = {}", dividend, divider, dividend / divider);

    let mut a = 433.0;
    let mut b = 6.0;
    println!("{} / {} = {}", a, b, a / b);

    a = 8.0;
    b = 1230.0;
    println!("{} / {} = {}", a, b, a / b);
}


