use std::mem;

fn main() {
    let mut grid = Grid::new();
    println!("grid uses {} bytes on the stack", mem::size_of_val(&grid));
}

fn main2() {
    let t: String = String::from("turn off 448,208,645,684");
    let v: Vec<usize> = t
        .split(" ")
        .last()
        .unwrap()
        .split(",")
        .map(|a| a.parse().unwrap())
        .collect();
    println!("{:?}", v);
    fourvar(v[0], v[1], v[2], v[3]);
}

fn fourvar(a: usize, b: usize, c: usize, d: usize) {
    println!("{} {} {} {}", a, b, c, d);
}

struct Grid {
    node: [[u32; 1000]; 1000],
}
impl Grid {
    fn new() -> Grid {
        Grid {
            node: [[0; 1000]; 1000],
        }
    }
}
