use std::fs;
use std::num::ParseIntError;
use std::str::FromStr;

fn main() {
    let filename: &str = "./src/bin/day02.txt";
    let data = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let mut paper_area = 0;
    let mut ribbon_length = 0;
    for d in data.lines() {
        let b: Box = d.parse().unwrap();
        paper_area += b.area() + b.slack();
        ribbon_length += b.bow() + b.ribbon();
    }
    println!("paper_area: {}", paper_area);
    println!("ribbon_length: {}", ribbon_length);
}

struct Box {
    l: i32,
    w: i32,
    h: i32,
}

impl Box {
    //fn new(l: i32, w: i32, h: i32) -> Box {
    //    //Box { l: l, w: w, h: h }
    //    Box { l, w, h }
    //}
    fn area(&self) -> i32 {
        return 2 * self.l * self.w + 2 * self.w * self.h + 2 * self.h * self.l;
    }
    fn slack(&self) -> i32 {
        let mut s = [self.l, self.w, self.h];
        s.sort();
        return s[0] * s[1];
    }
    fn bow(&self) -> i32 {
        return self.l * self.w * self.h;
    }
    fn ribbon(&self) -> i32 {
        let mut s = [self.l, self.w, self.h];
        s.sort();
        return s[0] * 2 + s[1] * 2;
    }
}

impl FromStr for Box {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let dims: Vec<&str> = s.split('x').collect();
        let l = dims[0].parse::<i32>()?;
        let w = dims[1].parse::<i32>()?;
        let h = dims[2].parse::<i32>()?;

        Ok(Box { l, w, h })
    }
}
