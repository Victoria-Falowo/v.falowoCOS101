use std::fs;

fn main() {
    fs::remove_file("data3.txt").expect("could not remove file");
    println!("file is removed");
}
