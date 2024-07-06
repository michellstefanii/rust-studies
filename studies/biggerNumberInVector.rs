fn biggest_number(vector: Vec<i32>) -> i32 {
    let mut bigger = 0;

    for a in vector {
        if a > bigger {
            bigger = a;
        }
    }

    bigger
}

fn main() {
    let vector = vec![10, 20, 30, 3000, 50, 60, 9999, 99, 200, 1, 500, 1000];
    println!("The biggest number is: {}", biggest_number(vector));
}
