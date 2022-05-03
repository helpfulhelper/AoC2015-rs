use std::fs;

fn main() {
    let filename: &str = "./src/bin/day05.txt";
    let data = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let mut count = 0;
    for d in data.lines() {
        if nice(d) {
            count += 1;
        }
    }
    println!("{}", count);
}

fn nice(s: &str) -> bool {
    !tabboo(s) && vows(s) && dubs(s)
}

fn dubs(s: &str) -> bool {
    let mut counter = 0;
    for (i, j) in s.chars().zip(s.chars().skip(1)) {
        if i == j {
            counter += 1;
        }
    }
    counter >= 1
}

fn vows(s: &str) -> bool {
    let mut counter = 0;
    for i in "aeiou".chars() {
        let count = s.matches(i).count();
        counter += count;
    }
    counter >= 3
}

// fn tabboo(s: &str) -> bool {
//     //if s.find("ab" || "cd" || "pq" || "xy") {
//     if s.contains("ab") || s.contains("cd") || s.contains("pq") || s.contains("xy") {
//         true
//     } else {
//         false
//     }
// }

fn tabboo(s: &str) -> bool {
    let bad = ["ab", "cd", "pq", "xy"];
    for i in bad.iter() {
        if s.contains(i) {
            return true;
        }
    }
    false
}
