use std::fs;

fn main() {
    let filename: &str = "./src/bin/day01.txt";
    let data = fs::read_to_string(filename).expect("Something went wrong reading the file");
    println!("{}", data);
}
