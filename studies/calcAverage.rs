fn calcula_media(grades: &[f64]) -> f64 {
    let mut calc = 0.0;
    for nota in grades {
        calc += *nota;
    }

    let result = calc / grades.len() as f64;
    return result;
}
fn main() {
    let grades = [10.0, 9.5, 8.0, 3.0, 2.0, 5.5];

    println!("The result is {}", calcula_media(&grades));
}
