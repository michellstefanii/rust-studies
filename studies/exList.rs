fn main() {
    println!("Please enter a sequence of real numbers:");

    let mut input = String::new();

    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    let mut numbers: Vec<f64> = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse::<f64>().expect("Please enter real numbers!"))
        .collect();

    let mut sum: f64 = 0.0;
    for num in &numbers {
        if num % 2.0 == 0.0 {
            sum = sum + num;
        }
    }

    println!("A soma dos números pares é: {}", sum);
}
