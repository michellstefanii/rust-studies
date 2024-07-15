mod verify_permutation;
use verify_permutation::is_permutation;

fn main() {
    let str1 = "abc";
    let str2 = "bca";

    if is_permutation(str1, str2) {
        println!("Is permutation");
    } else {
        println!("It's not a permutation");
    }
}
