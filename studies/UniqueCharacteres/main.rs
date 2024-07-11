mod verify_characteres;

fn main() {
    let string = "cateto";

    if verify_characteres::have_unique_characteres(string) {
        println!("The string has all unique characters.");
    } else {
        println!("The string does not have all unique characters.");
    }
}
