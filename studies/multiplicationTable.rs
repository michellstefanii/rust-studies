use std::io;

fn convert_to_int(data_input: &String) -> i32 {
    let x = data_input.trim().parse::<i32>().unwrap();
    x
}

fn main() {
    let mut number = String::new();
    io::stdin()
        .read_line(&mut number)
        .expect("Read input error!");
    let number_int = convert_to_int(&number);

    for x in 1..=10 {
        let calc = x * number_int;
        println!("{} x {} = {}", number_int, x, calc);
    }
}
