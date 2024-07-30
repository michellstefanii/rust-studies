fn compress_string(a: &str) -> String {
    let mut concatenated = String::new();
    let mut prev_char: Option<char> = None;
    let mut count = 0;

    for ca in a.chars() {
        if let Some(prev) = prev_char {
            if prev == ca {
                count += 1;
                continue;
            } else {
                concatenated.push(prev);
                concatenated.push_str(&count.to_string());
            }
        }

        prev_char = Some(ca);
        count = 1;
    }

    if let Some(prev) = prev_char {
        concatenated.push(prev);
        concatenated.push_str(&count.to_string());
    }

    concatenated
}

fn main() {
    let str1: &str = "aabcccccaaa";
    let compressed_string: String = compress_string(str1);
    println!("Original {}", str1);
    println!("Compressed {}", compressed_string);
}
