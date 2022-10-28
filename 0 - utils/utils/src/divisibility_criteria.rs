
pub fn _divisible_by(value: i32, number: i32) -> bool {
    match number {
        2 => value % 2 == 0,
        3 => divisible_by_3(value),
        4 => divisible_by_4(value),
        5 => divisible_by_5(value),
        6 => false,
        7 => false,
        8 => false,
        9 => false,
        _ => false
    }
}

fn divisible_by_3(value: i32) -> bool {
    let number = value.to_string();
    let mut total = 0;

    for item in number.chars() {
        let num :i32 = 
            item.to_string().trim()
                .parse().expect("Enter a valid number.");
        
        total += num;
    }

    total % 3 == 0
}

fn divisible_by_4(value: i32) -> bool {
    let number = value.to_string();
    let offset;

    if number.len() >= 2 {
        offset = number.len() - 2;
    } else {
        offset = number.len() - 1;
    }

    let part = &number[offset..];
    let number :i32 = part.to_string().trim()
        .parse().expect("Enter a valid number.");
    
    number % 4 == 0
}

fn divisible_by_5(value: i32) -> bool {
    let number = value.to_string();
    let offset = number.len() - 1;
    let part   = &number[offset..];
    
    let number :i32 = part.to_string().trim()
        .parse().expect("Enter a valid number.");

    match number {
        0 => true,
        5 => true,
        _ => false
    }
}


