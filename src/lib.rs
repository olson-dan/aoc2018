extern crate aoc_runner;
#[macro_use]
extern crate aoc_runner_derive;
use std::collections::{HashMap, HashSet};

#[aoc_generator(day1)]
pub fn input_generator_day1(input: &str) -> Vec<i64> {
    input.lines().map(|x| x.trim().parse().unwrap()).collect()
}

#[aoc(day1, part1)]
pub fn solve_day1_part1(input: &[i64]) -> i64 {
    input.iter().sum()
}

#[aoc(day1, part2)]
pub fn solve_day1_part2(input: &[i64]) -> i64 {
    let mut set: HashSet<i64> = HashSet::new();
    set.insert(0);
    let mut accumulator = 0;
    for i in input.iter().cycle() {
        accumulator = accumulator + i;
        if !set.insert(accumulator) {
            return accumulator;
        }
    }
    unreachable!();
}

#[aoc_generator(day2)]
pub fn input_generator_day2(input: &str) -> Vec<String> {
    input.lines().map(|x| x.trim().to_string()).collect()
}

#[aoc(day2, part1)]
pub fn solve_day2_part1(input: &[String]) -> u64 {
    let mut threes = 0;
    let mut twos = 0;
    for x in input {
        let mut map = HashMap::new();
        for c in x.chars() {
            let value = map.entry(c).or_insert(0);
            *value = *value + 1;
        }
        if map.iter().any(|(_, x)| *x == 2) {
            twos += 1
        };
        if map.iter().any(|(_, x)| *x == 3) {
            threes += 1
        };
    }
    threes * twos
}

#[aoc(day2, part2)]
pub fn solve_day2_part2(input: &[String]) -> String {
    for (i, x) in input.iter().enumerate() {
        for y in input[i..].iter() {
            let mut different = 0;
            let mut same = String::new();
            for (a, b) in x.chars().zip(y.chars()) {
                if a == b {
                    same.push(a);
                } else {
                    different += 1;
                }
            }
            if different == 1 {
                return same;
            }
        }
    }
    unreachable!()
}

pub struct Rect {
    claim: u32,
    x: i32,
    y: i32,
    w: i32,
    h: i32,
}
#[aoc_generator(day3)]
pub fn input_generator_day3(input: &str) -> Vec<Rect> {
    let mut rects = Vec::new();
    for line in input.lines() {
        let mut it = line.split_whitespace();
        let claim = it.next().unwrap();
        let _ = it.next(); // @
        let xy = it.next().unwrap().trim_matches(':');
        let (x, y) = xy.split_at(xy.find(',').unwrap());
        let wh = it.next().unwrap();
        let (w, h) = wh.split_at(wh.find('x').unwrap());
        rects.push(Rect {
            claim: claim[1..].parse().unwrap(),
            x: x.parse().expect("x"),
            y: y[1..].parse().expect("y"),
            w: w.parse().expect("w"),
            h: h[1..].parse().expect("h"),
        })
    }
    rects
}

#[aoc(day3, part1)]
pub fn solve_day3_part1(input: &[Rect]) -> usize {
    let mut map: HashMap<(i32, i32), u32> = HashMap::new();
    for rect in input {
        for x in rect.x..rect.x + rect.w {
            for y in rect.y..rect.y + rect.h {
                let value = map.entry((x, y)).or_insert(0);
                *value = *value + 1;
            }
        }
    }
    map.iter().filter(|(_, y)| **y > 1).count()
}

#[aoc(day3, part2)]
pub fn solve_day3_part2(input: &[Rect]) -> u32 {
    let mut map: HashMap<(i32, i32), u32> = HashMap::new();
    for rect in input {
        for x in rect.x..rect.x + rect.w {
            for y in rect.y..rect.y + rect.h {
                let value = map.entry((x, y)).or_insert(0);
                *value = *value + 1;
            }
        }
    }
    'a: for rect in input {
        for x in rect.x..rect.x + rect.w {
            for y in rect.y..rect.y + rect.h {
                let value = map.entry((x, y)).or_insert(0);
                if *value != 1 {
                    continue 'a;
                }
            }
        }
        return rect.claim;
    }
    unreachable!();
}
aoc_lib!{ year = 2018 }
