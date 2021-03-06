use std::collections::HashMap;
use std::fs;

fn main() {
    let filename: &str = "./src/bin/day03.txt";
    let data = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let mut santa = Pos { x: 0, y: 0 };
    let mut robot = Pos { x: 0, y: 0 };
    let mut houses = HashMap::new();
    houses.insert((0, 0), 0);

    for (step, dir) in data.chars().enumerate() {
        let s = step % 2;
        match s {
            0 => {
                santa.mov(dir);
                let count = houses.entry(santa.get()).or_insert(0);
                *count += 1;
            }
            1 => {
                robot.mov(dir);
                let count = houses.entry(robot.get()).or_insert(0);
                *count += 1;
            }
            _ => (),
        }
    }
    println!("santa's last position: {:?}", santa.get());
    println!("Robo-santa's last position: {:?}", robot.get());
    println!("total houses visited: {}", houses.len());
}

#[derive(Debug)]
struct Pos {
    x: i32,
    y: i32,
}

impl Pos {
    fn mov(&mut self, dir: char) {
        match dir {
            '^' => self.y += 1,
            'v' => self.y -= 1,
            '<' => self.x -= 1,
            '>' => self.x += 1,
            _ => (),
        }
    }

    fn get(&self) -> (i32, i32) {
        return (self.x, self.y);
    }
}
