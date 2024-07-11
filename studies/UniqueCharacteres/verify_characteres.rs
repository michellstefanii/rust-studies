use std::collections::HashSet;

pub fn have_unique_characteres(input: &str) -> bool {
    let mut seen = HashSet::new();
    !input.chars().any(|c| !seen.insert(c))
}
