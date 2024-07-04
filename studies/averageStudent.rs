use std::io;

fn convert_to_int(data_input: &String) -> i32 {
    let x = data_input.trim().parse::<i32>().unwrap();
    x
}

fn main() {
    let mut input_qtd_average = String::new();

    println!("Enter the number of students:");
    io::stdin()
        .read_line(&mut input_qtd_average)
        .expect("Read input number of students error!");

    let mut qtd_average_int = convert_to_int(&input_qtd_average);
    let mut qtd_dp = 0;

    while qtd_average_int > 0 {
        let mut average = String::new();
        println!("Enter the student average {}", qtd_average_int);
        io::stdin()
            .read_line(&mut average)
            .expect("Read input student average error!");
        let average_int = convert_to_int(&average);

        if average_int >= 3 && average_int < 6 {
            qtd_dp += 1;
        }

        qtd_average_int = qtd_average_int - 1;
    }

    println!("Number of students in recovery {}", qtd_dp);
}
