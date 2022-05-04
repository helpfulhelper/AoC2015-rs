use std::fs;

fn main() {
    let filename: &str = "./src/bin/day06.txt";
    let data = fs::read_to_string(filename).expect("Something went wrong reading the file");
    //println!("{}", data);
    //let mut grid = Grid::new();
    let mut grid: Box<Grid> = Box::new(Grid {
        node: [[false; 1000]; 1000],
        brightness: [[0; 1000]; 1000],
    });
    for d in data.lines() {
        //remove pointless text and reformat
        let t = d.replace(" through ", ",");
        //get command string
        let mut command: Vec<&str> = t.split(" ").collect();
        let nums: Vec<usize> = command
            .pop()
            .unwrap()
            .split(",")
            .map(|a| a.parse::<usize>().unwrap())
            .collect();
        let command_str = command.join(" ");
        grid.apply(command_str.as_str(), nums[0], nums[1], nums[2], nums[3]);
        grid.apply_bright(command_str.as_str(), nums[0], nums[1], nums[2], nums[3]);
    }
    println!("Lit Lights: {}", grid.num_lit());
    println!("How Bright: {}", grid.how_lit());
}

#[allow(dead_code)]
struct Grid {
    node: [[bool; 1000]; 1000],
    brightness: [[u16; 1000]; 1000],
}
#[allow(dead_code)]
impl Grid {
    fn new() -> Grid {
        Grid {
            node: [[false; 1000]; 1000],
            brightness: [[0; 1000]; 1000],
        }
    }

    fn apply(&mut self, command: &str, x1: usize, y1: usize, x2: usize, y2: usize) {
        for y in y1..=y2 {
            for x in x1..=x2 {
                match command {
                    "toggle" => self.node[x][y] = !self.node[x][y],
                    "turn on" => self.node[x][y] = true,
                    "turn off" => self.node[x][y] = false,
                    _ => (),
                }
            }
        }
    }
    fn apply_bright(&mut self, command: &str, x1: usize, y1: usize, x2: usize, y2: usize) {
        for y in y1..=y2 {
            for x in x1..=x2 {
                match command {
                    "toggle" => self.brightness[x][y] += 2,
                    "turn on" => self.brightness[x][y] += 1,
                    "turn off" => {
                        if self.brightness[x][y] > 0 {
                            self.brightness[x][y] -= 1
                        } else {
                            self.brightness[x][y] = 0
                        }
                    }
                    _ => (),
                }
            }
        }
    }

    fn num_lit(&self) -> usize {
        let mut counter = 0;
        for y in 0..1000 {
            for x in 0..1000 {
                if self.node[x][y] {
                    counter += 1;
                }
            }
        }
        counter
    }
    fn how_lit(&self) -> u64 {
        let mut counter: u64 = 0;
        for y in 0..1000 {
            for x in 0..1000 {
                counter += self.brightness[x][y] as u64;
            }
        }
        counter
    }
}

#[test]
fn test_part1() {
    let mut grid = Grid::new();
    grid.apply("turn on", 0, 0, 999, 999);
    assert_eq!(grid.num_lit(), 1_000_000);
    grid.apply("toggle", 0, 0, 999, 0);
    assert_eq!(grid.num_lit(), 999_000);
    grid.apply("off", 499, 499, 500, 500);
    assert_eq!(grid.num_lit(), 998_996);
}
