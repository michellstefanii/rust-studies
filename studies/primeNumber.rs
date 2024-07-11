use std::f64;
use std::io;

fn convert_to_int(data_input: &String) -> i32 {
    let x = data_input.trim().parse::<i32>().unwrap();
    x
}

fn eh_primo(num: &i32) -> bool {
    if *num <= 1 {
        return false;
    };
    if *num <= 3 {
        return true;
    };

    if *num % 2 == 0 || *num % 3 == 0 {
        return false;
    };

    let limite = (*num as f64).sqrt().ceil() as i32;

    for x in (5..=limite).step_by(6) {
        if (*num % x) == 0 || num % (x + 2) == 0 {
            return false;
        };
    }

    true
}

fn main() {
    let mut number = String::new();
    io::stdin()
        .read_line(&mut number)
        .expect("Read input error!");
    let number_int = convert_to_int(&number);
    println!("Prime number ? {}", eh_primo(&number_int));
}
