use std::io;

fn convert_to_int(data_input: &String) -> i32 {
    let x = data_input.trim().parse::<i32>().unwrap();
    x
}

fn main() {
    let mut sum = 0;
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Ocorreu um erro ao ler o valor");
    let mut value_i32 = convert_to_int(&input);

    while value_i32 != 0 {
        let mut r = value_i32 % 10;
        sum += r;
        value_i32 = value_i32 / 10;
        println!("sum {}, value {}", sum, value_i32)
    }
    println!("O valor da soma dos digitos {}", sum);
}
