use itertools::Itertools;
use std::collections::HashMap;

#[derive(Debug)]
enum Gate {
    NumValue(u16),
    Value(&'static str),
    Not(&'static str),
    And(&'static str, &'static str),
    Or(&'static str, &'static str),
    LShift(&'static str, &'static str),
    RShift(&'static str, &'static str),
}
fn main() {
    let data = include_str!("day07.txt");
    let circuits = build_map(data);
    println!("Circuits built without issue. Calculating a: ");
    let foo = find_val(&circuits, "a");
    println!("a is {:#?}", foo);
}

fn build_map(data: &'static str) -> HashMap<&str, Gate> {
    let hmap: HashMap<&str, Gate> =
        data.lines()
            .map(gparse)
            .fold(HashMap::new(), |mut acc, (desc, name)| {
                acc.insert(name, desc);
                acc
            });
    return hmap;
}

fn gparse(line: &'static str) -> (Gate, &str) {
    let (desc, name) = line.split_once(" -> ").unwrap();
    if let Some((a, op, b)) = desc.split(' ').collect_tuple() {
        return match op {
            "AND" => (Gate::And(a, b), name),
            "OR" => (Gate::Or(a, b), name),
            "LSHIFT" => (Gate::LShift(a, b), name),
            "RSHIFT" => (Gate::RShift(a, b), name),
            _ => unreachable!(),
        };
    }
    if let Some((_, a)) = desc.split(' ').collect_tuple() {
        return (Gate::Not(a), name);
    }
    (Gate::Value(desc), name)
}

fn exec(map: &HashMap<&str, Gate>, key: &str, cache: &mut HashMap<&str, u16>) -> u16 {
    //println!("Running value with: {}", key);
    //Immediately check if our wanted key is actually a number. Return if so.
    if let Ok(num) = key.parse::<u16>() {
        return num;
    }
    match map[key] {
        Gate::NumValue(v) => v,
        Gate::Value(v) => memoize_exec(map, v, cache),
        Gate::Not(v) => !memoize_exec(map, v, cache),
        Gate::And(l, r) => memoize_exec(map, l, cache) & memoize_exec(map, r, cache),
        Gate::Or(l, r) => memoize_exec(map, l, cache) | memoize_exec(map, r, cache),
        Gate::LShift(l, r) => memoize_exec(map, l, cache) << memoize_exec(map, r, cache),
        Gate::RShift(l, r) => memoize_exec(map, l, cache) >> memoize_exec(map, r, cache),
    }
}

fn memoize_exec(
    map: &HashMap<&str, Gate>,
    key: &'static str,
    cache: &mut HashMap<&str, u16>,
) -> u16 {
    if let Some(&value) = cache.get(key) {
        return value;
    };
    let result = exec(map, key, cache);
    cache.insert(key, result);
    result
}
fn find_val(map: &HashMap<&str, Gate>, val: &'static str) -> u16 {
    let mut cache: HashMap<&str, u16> = HashMap::new();
    exec(map, val, &mut cache)
}

#[test]
fn test_logic() {
    let data: &'static str = include_str!("day07.test");
    let circuits = build_map(data);

    println!("min: {:?}", find_val(&circuits, "min"));
    assert!(find_val(&circuits, "min") == 0);

    println!("one: {:?}", find_val(&circuits, "one"));
    assert!(find_val(&circuits, "one") == 1);

    println!("five: {:?}", find_val(&circuits, "five"));
    assert!(find_val(&circuits, "five") == 21845);

    println!("aaaa: {:?}", find_val(&circuits, "aaaa"));
    assert!(find_val(&circuits, "aaaa") == 43690);

    println!("max: {:?}", find_val(&circuits, "max"));
    assert!(find_val(&circuits, "max") == 65535);

    println!("val: {:?}", find_val(&circuits, "val"));
    assert!(find_val(&circuits, "val") == 21845);

    println!("not: {:?}", find_val(&circuits, "not"));
    assert!(find_val(&circuits, "not") == 21845);

    println!("and: {:?}", find_val(&circuits, "and"));
    assert!(find_val(&circuits, "and") == 0);

    println!("or: {:?}", find_val(&circuits, "or"));
    assert!(find_val(&circuits, "or") == 65535);

    println!("lsh: {:?}", find_val(&circuits, "lsh"));
    assert!(find_val(&circuits, "lsh") == 43690);

    println!("rsh: {:?}", find_val(&circuits, "rsh"));
    assert!(find_val(&circuits, "rsh") == 21845);

    println!("sh: {:?}", find_val(&circuits, "sh"));
    assert!(find_val(&circuits, "sh") == 0);

    println!("fm: {:?}", find_val(&circuits, "fm"));
    assert!(find_val(&circuits, "fm") == 21845);

    println!("lm: {:?}", find_val(&circuits, "lm"));
    assert!(find_val(&circuits, "lm") == 32768);
}
