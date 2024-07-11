use std::io;

fn convert_to_int(data_input: &String) -> i32 {
    let x = data_input.trim().parse::<i32>().unwrap();
    x
}
fn main() {
    let mut list: Vec<i32> = Vec::new();

    let mut number = String::new();
    println!("Enter the number of numbers to be entered");
    io::stdin()
        .read_line(&mut number)
        .expect("Read input error!");
    let number_int = convert_to_int(&number);

    for x in 1..=number_int {
        let mut number_input = String::new();

        println!("Enter the number {}", x);
        io::stdin()
            .read_line(&mut number_input)
            .expect("Read input error!");
        let number_input_int: i32 = convert_to_int(&number_input);

        if number_input_int % 2 == 0 {
            list.push(number_input_int);
        }
    }

    let mut calc = 0;
    for y in list {
        calc = calc + y;
    }

    println!("The Result is {}", calc)
}
