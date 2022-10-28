
pub fn _divisible_by(value: i32, number: i32) -> bool {
    match number {
        0 => false,
        1 => false,
        2 => value % 2 == 0,
        3 => divisible_by_3(value),
        4 => false,
        5 => false,
        6 => false,
        7 => false,
        8 => false,
        9 => false,
        _ => false,
    }
}

fn divisible_by_3(value: i32) -> bool {
    let number :String = value.to_string();
    let mut total = 0;

    for item in number.chars() {
        let num :i32 = 
            item.to_string().trim()
                .parse().expect("Enter a valid number.");
        
        total += num;
    }

    total % 3 == 0
}


