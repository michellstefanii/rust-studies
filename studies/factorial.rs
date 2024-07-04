use std::io;

fn convert_to_int(data_input: &String) -> i32 {
    let x = data_input.trim().parse::<i32>().unwrap();
    x
}

fn main() {
    let mut input_factorial = String::new();
    io::stdin()
        .read_line(&mut input_factorial)
        .expect("Read input error!");
    let mut factorial = 1;
    let mut factorial_int = convert_to_int(&input_factorial);

    while factorial_int > 1 {
        factorial = factorial * factorial_int;
        factorial_int = factorial_int - 1;
    }

    println!("Factorial result {}", factorial);
}
