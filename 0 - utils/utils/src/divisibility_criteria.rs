
pub fn divisible_by(value: i32, number: i32) -> bool {
    match number {
        2  => value % 2 == 0,
        3  => divisible_by_3(value),
        4  => divisible_by_4(value),
        5  => divisible_by_5(value),
        6  => value % 2 == 0 && divisible_by_3(value),
        7  => divisible_by_7(value),
        8  => divisible_by_8(value),
        9  => divisible_by_9(value),
        10 => divisible_by_10(value),
        11 => false,
        12 => false,
        15 => false,
        25 => false,
        _  => false
    }
}

fn sum_and_rem(value: i32, current :i32) -> bool {
    let number = value.to_string();
    let mut total = 0;

    for item in number.chars() {
        let num :i32 = 
            item.to_string().trim()
                .parse().expect("Enter a valid number.");
        
        total += num;
    }

    total % current == 0
}

fn divisible_by_3(value: i32) -> bool {
    sum_and_rem(value, 3)
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

fn divisible_by_7(value :i32) -> bool {
    let number = value.to_string();
    let len = number.len() - 1;
    let mut s = String::from("");
    let mut counter1 = 0;
    let mut counter2 = 0;
    let mut numbers :Vec<i32> = Vec::new();

    // Separate numbers into 3-digit blocks and divide by 7.
    for item in number.chars().rev() {
        let num = item.to_string();
        s.push_str(&num[..]);

        if counter1 == 2 || counter2 == len {
            let nums :String = s.chars().rev().collect();
            let num : i32 = nums.to_string().parse()
                .expect("Invalid number.");
            s.clear();
            counter1 = 0;
            numbers.push(num % 7);
        } else {
            counter1 += 1;
        }

        counter2 += 1;
    }

    // Processes the value of the remainder of the division and their signs.
    let mut numbers :Vec<i32> = numbers.into_iter().rev().collect();
    let mut flag = true;
    let mut total = 0;

    for num in &mut numbers {
        let mut number = String::from("");

        if flag == true {
            number.push_str(&format!("{}", num));
            flag = false;
        } else {
            number.push_str(&format!("-{}", num));
            flag = true;
        }

        let number :i32 = number.to_string()
            .parse().expect("Invalid number.");
        total += number;
    }

    total % 7 == 0
}

fn divisible_by_8(value :i32) -> bool {
    let number = value.to_string();
    let part;

    if number.len() > 3 {
        let len = number.len() - 3;
        part = &number[len..];
    } else {
        part = &number[..];
    }

    let total :i32 = part.to_string()
        .parse().expect("Invalid number.");

    total % 8 == 0
}

fn divisible_by_9(value: i32) -> bool {
    sum_and_rem(value, 9)
}

fn divisible_by_10(value: i32) -> bool {
    let number = value.to_string();
    let offset = number.len() - 1;
    let part   = &number[offset..];
    
    let number :i32 = part.to_string().trim()
        .parse().expect("Enter a valid number.");

    number == 0
}


