use std::fs;

fn main() {
    let filename: &str = "./src/bin/day01.txt";
    let data = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let mut floor = 0;
    let mut count = 0;

    for d in data.chars() {
        floor += if d == '(' { 1 } else { -1 };
        count += 1;
        if floor == -1 {
            println!("Step {}", count);
        }
    }
    println!("Final Floor: {}", floor);
    //println!("{}", data);
}
