extern crate aoc_runner;
#[macro_use]
extern crate aoc_runner_derive;
extern crate regex;

use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::ops::Range;

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
    x: u32,
    y: u32,
    w: u32,
    h: u32,
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
    let mut map: HashMap<(u32, u32), u32> = HashMap::new();
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
    let mut map: HashMap<(u32, u32), u32> = HashMap::new();
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

#[derive(Clone, Debug, Default)]
pub struct Guard {
    id: u32,
    sleeps: Vec<Range<u32>>,
    date: String,
}

#[aoc_generator(day4)]
pub fn input_generator_day4(input: &str) -> Vec<Guard> {
    let re = Regex::new(
        r"^\[(.*) (\d{2}):(\d{2})\] (Guard #(\d+) begins shift|(wakes up)|(falls asleep))$",
    ).unwrap();

    let mut guards = Vec::new();

    #[derive(Clone)]
    enum State {
        Change(Option<u32>),
        Awake(u32),
        Asleep(u32),
    }
    let mut current_guard = Guard::default();
    let mut state = State::Change(None);
    let mut input: Vec<&str> = input.lines().collect();
    input.sort_unstable();
    for l in &input {
        let cap = re.captures(l).unwrap();
        let date = cap.get(1).unwrap().as_str();
        let time: u32 = cap
            .get(3)
            .unwrap()
            .as_str()
            .parse()
            .expect("couldn't parse time");
        let guard: Option<u32> = cap
            .get(5)
            .map(|x| x.as_str().parse().expect("couldn't parse guard"));
        let wakes = cap.get(6).is_some();
        let sleeps = cap.get(7).is_some();
        let new_state = match (wakes, sleeps) {
            (false, false) => State::Change(guard),
            (true, false) => State::Awake(time),
            (false, true) => State::Asleep(time),
            _ => unreachable!(),
        };
        let next_state = match (&state, &new_state) {
            (State::Asleep(_), State::Change(Some(_))) => unreachable!(),
            (State::Asleep(start), State::Awake(end)) => {
                current_guard.sleeps.push(Range {
                    start: *start,
                    end: *end,
                });
                State::Awake(*end)
            }
            (State::Awake(_), State::Change(Some(id))) => {
                if !current_guard.date.is_empty() {
                    guards.push(current_guard.clone());
                }
                current_guard.date = date.to_string();
                current_guard.id = *id;
                current_guard.sleeps = Vec::new();
                State::Change(None)
            }
            _ => new_state.clone(),
        };
        state = next_state;
    }
    guards
}

#[aoc(day4, part1)]
pub fn solve_day4_part1(input: &[Guard]) -> u32 {
    let mut total_sleep: HashMap<u32, u32> = HashMap::new();
    for guard in input {
        let amount: u32 = guard.sleeps.iter().map(|x| x.len() as u32).sum();
        let sleep = total_sleep.entry(guard.id).or_insert(0);
        *sleep = *sleep + amount;
    }
    let (max_id, _) = total_sleep.iter().max_by_key(|(_, ref x)| *x).unwrap();

    let mut total_minutes: HashMap<u32, u32> = HashMap::new();
    for guard in input {
        if guard.id == *max_id {
            for sleep in &guard.sleeps {
                for minute in sleep.start..sleep.end {
                    *total_minutes.entry(minute).or_insert(0) += 1;
                }
            }
        }
    }
    let (max_minute, _) = total_minutes.iter().max_by_key(|(_, ref x)| *x).unwrap();
    max_id * max_minute
}

#[aoc(day4, part2)]
pub fn solve_day4_part2(input: &[Guard]) -> u32 {
    let mut minutes = vec![HashMap::<u32, u32>::new(); 59];
    for guard in input {
        for sleep in &guard.sleeps {
            for minute in sleep.start..sleep.end {
                *minutes[minute as usize].entry(guard.id).or_insert(0) += 1;
            }
        }
    }
    let max_minutes: Vec<(u32, u32)> = minutes
        .iter()
        .map(|x| {
            let (guard_id, times) = x.iter().max_by_key(|(_, ref y)| *y).unwrap();
            (*guard_id, *times)
        }).collect();
    let (guard_id, times) = max_minutes.iter().max_by_key(|(_, y)| y).unwrap();
    let (minute, _) = max_minutes
        .iter()
        .enumerate()
        .find(|(_, x)| *x == &(*guard_id, *times))
        .unwrap();
    guard_id * minute as u32
}

#[aoc_generator(day5)]
pub fn input_generator_day5(input: &str) -> String {
    input.trim().to_string()
}

#[aoc(day5, part1)]
pub fn solve_day5_part1(input: &str) -> usize {
    let mut v = String::new();
    for x in input.chars() {
        if let Some(popped) = v.pop() {
            if x == popped || !x.eq_ignore_ascii_case(&popped) {
                v.push(popped);
                v.push(x);
            }
        } else {
            v.push(x);
        }
    }
    return v.len();
}

#[aoc(day5, part2)]
pub fn solve_day5_part2(input: &str) -> usize {
    "abcdefghijklmnopqrstuvwxyz"
        .chars()
        .map(|c| {
            let mut s = input.to_string();
            s.retain(|x| !x.eq_ignore_ascii_case(&c));
            solve_day5_part1(&s)
        }).min()
        .unwrap()
}
aoc_lib!{ year = 2018 }
