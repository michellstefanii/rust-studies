use std::io;

fn is_numeric(s: &str) -> bool {
    s.chars().all(|c| c.is_digit(10))
}

fn main() {
    let mut input: String = String::new();

    println!("Insert a number to verify a palindrome");
    io::stdin().read_line(&mut input).expect("Err");

    let input_parse = input.trim();

    if is_numeric(&input_parse) {
        let input_reverse: String = input_parse.chars().rev().collect();

        if input_reverse.to_string() == input_parse {
            return println!("{} is a palindrome", input_parse);
        } else {
            return println!("{} not is a palindrome", input_parse);
        }
    } else {
        return println!("Only numbers is a palindrome");
    }
}
