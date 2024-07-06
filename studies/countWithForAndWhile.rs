fn count(num: i32) {
    for a in 1..(num + 1) {
        println!("{}", a)
    }
}

fn count_down(num: i32) {
    let mut counter = num;

    while counter > 0 {
        println!("{}", counter);
        counter = counter - 1;
    }
}

fn main() {
    println!("Counting up to 10:");
    count(10);
    println!("Counting down from 10:");
    count_down(10);
}
