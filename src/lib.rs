extern crate aoc_runner;
#[macro_use]
extern crate aoc_runner_derive;
use std::collections::{HashMap, HashSet};

#[aoc_generator(day1)]
pub fn input_generator_day1(input: &str) -> Vec<i64> {
    input.lines().map(|x| x.trim().parse().unwrap()).collect()
}

#[aoc_generator(day2)]
pub fn input_generator_day2(input: &str) -> Vec<String> {
    input.lines().map(|x| x.trim().to_string()).collect()
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

aoc_lib!{ year = 2018 }
