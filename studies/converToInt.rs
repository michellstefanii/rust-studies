use std::io;

fn convert_to_int(data_input: &String) -> i32 {
    let x = data_input.trim().parse::<i32>().unwrap();
    x
}

fn main() {
    let mut number1 = String::new();
    io::stdin()
        .read_line(&mut number1)
        .expect("Ocorreu um erro ao ler o valor");
    let mut number2 = String::new();
    io::stdin()
        .read_line(&mut number2)
        .expect("Ocorreu um erro ao ler o valor");

    if convert_to_int(&number1) > convert_to_int(&number2) {
        println!("Número {} é maior que {}", number1, number2)
    } else {
        println!("Número {} é menor ou igual ao {}", number1, number2)
    }
}
