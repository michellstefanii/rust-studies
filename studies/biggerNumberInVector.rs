fn biggest_number(array: &[i32]) -> i32 {
    let mut bigger = array[0];

    for &a in array.iter() {
        if a > bigger {
            bigger = a;
        }
    }

    bigger
}

fn main() {
    let array = [10, 20, 30, 3000, 50, 60, 9999, 99, 200, 1, 500, 1000];
    println!("The biggest number is: {}", biggest_number(&array));
}
