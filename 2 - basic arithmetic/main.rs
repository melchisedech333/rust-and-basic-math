
fn main() {
    println!("Operations: \n");
    operations();

    println!("\n\n\nSignals: \n");
    signals();
}

fn operations() {
    let a = |x,y| x + y;
    println!("a: {}", a(10, 20));

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

    println!("+: {}", calc(b, c, '+'));
    println!("-: {}", calc(b, c, '-'));
    println!("*: {}", calc(b, c, '*'));
    println!("/: {}", calc(b, c, '/'));
}

fn signals() {
    let show = |a,b| println!("{a} = {b}");

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


