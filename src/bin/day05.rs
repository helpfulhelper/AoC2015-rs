use std::fs;

fn main() {
    let filename: &str = "./src/bin/day05.txt";
    let data = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let mut count = 0;
    for d in data.lines() {
        if nice2(d) {
            count += 1;
        }
    }
    println!("{}", count);
}

fn nice(s: &str) -> bool {
    !tabboo(s) && vowels(s) && dubs(s, 1)
}

fn nice2(s: &str) -> bool {
    dubs(s, 2) && pairs(s)
}

fn pairs(s: &str) -> bool {
    let mut z = s.chars().peekable();
    while z.peek().is_some() {
        let chunk: String = z.by_ref().take(2).collect();
        if chunk.len() == 2 && s.split(&chunk).count() - 1 > 1 {
            return true;
        };
    }
    z = s.chars().peekable();
    let _z: String = z.by_ref().take(1).collect();
    while z.peek().is_some() {
        let chunk: String = z.by_ref().take(2).collect();
        if chunk.len() == 2 && s.split(&chunk).count() - 1 > 1 {
            return true;
        };
    }
    false
}

fn dubs(s: &str, skip: usize) -> bool {
    let mut counter = 0;
    for (i, j) in s.chars().zip(s.chars().skip(skip)) {
        if i == j {
            counter += 1;
        }
    }
    counter >= 1
}

fn vowels(s: &str) -> bool {
    let mut counter = 0;
    for i in "aeiou".chars() {
        let count = s.matches(i).count();
        counter += count;
    }
    counter >= 3
}

fn tabboo(s: &str) -> bool {
    let bad = ["ab", "cd", "pq", "xy"];
    for i in bad.iter() {
        if s.contains(i) {
            return true;
        }
    }
    false
}

#[test]
fn test_dubs() {
    assert_eq!(dubs("abcdefgg", 1), true);
    assert_eq!(dubs("abcdefgh", 1), false);
    assert_eq!(dubs("abaca", 2), true);
    assert_eq!(dubs("abaca", 2), true);
}

#[test]
fn test_dubdubs() {
    assert_eq!(pairs("aaa"), false);
    assert_eq!(pairs("aabaa"), true);
    assert_eq!(pairs("aaaa"), true);
    assert_eq!(pairs("ieodomkazucvgmuy"), false);
}
