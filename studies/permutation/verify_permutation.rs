pub fn is_permutation(str1: &str, str2: &str) -> bool {
    let sorted_str1 = sort_string(str1);
    let sorted_str2 = sort_string(str2);

    sorted_str1 == sorted_str2
}

fn sort_string(s: &str) -> String {
    let mut chars: Vec<char> = s.chars().collect();
    chars.sort();
    chars.into_iter().collect()
}
