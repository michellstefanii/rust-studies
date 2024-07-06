fn main() {
    let mut a = 15;
    let mut b = 40;

    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }

    println!("Greatest common divisor between 15 and 40 is {}", a)
}
