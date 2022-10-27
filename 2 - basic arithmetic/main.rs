
fn main() {
    let a = |x,y| x + y;
    println!("a: {}\n", a(10, 20));

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


