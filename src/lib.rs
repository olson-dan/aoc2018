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
    )
    .unwrap();

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
        })
        .collect();
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
        })
        .min()
        .unwrap()
}

#[aoc_generator(day6)]
pub fn input_generator_day6(input: &str) -> Vec<(i32, i32)> {
    input
        .lines()
        .map(|x| {
            let (a, b) = x.trim().split_at(x.find(',').unwrap());
            (a.parse().unwrap(), b[2..].parse().unwrap())
        })
        .collect()
}

#[aoc(day6, part1)]
pub fn solve_day6_part1(input: &[(i32, i32)]) -> usize {
    let max_x = input.iter().max_by_key(|(x, _)| x).unwrap().0;
    let max_y = input.iter().max_by_key(|(_, y)| y).unwrap().1;
    let mut map: HashMap<(i32, i32), i32> = HashMap::new();
    let mut inf: HashSet<usize> = HashSet::new();
    for x in 0..=max_x {
        for y in 0..=max_y {
            let entry = map.entry((x, y)).or_insert(-1);
            let distances: Vec<i32> = input
                .iter()
                .map(|(a, b)| (x - a).abs() + (y - b).abs())
                .collect();
            let (index, min_distance) = distances
                .iter()
                .enumerate()
                .min_by_key(|(_, ref d)| *d)
                .unwrap();
            let matching = distances.iter().filter(|d| *d == min_distance).count();
            if matching == 1 {
                *entry = index as i32;
            }
            // Blacklist anything on the edges, its area will be infinite.
            if x == 0 || y == 0 || x == max_x || y == max_y {
                inf.insert(index);
            }
        }
    }
    let mut counts: Vec<usize> = (0..input.len())
        .into_iter()
        .map(|i| map.iter().filter(|(_, x)| **x == i as i32).count())
        .collect();
    for i in inf {
        counts[i] = 0;
    }
    *counts.iter().max().unwrap()
}

#[aoc(day6, part2)]
pub fn solve_day6_part2(input: &[(i32, i32)]) -> usize {
    let max_x = input.iter().max_by_key(|(x, _)| x).unwrap().0;
    let max_y = input.iter().max_by_key(|(_, y)| y).unwrap().1;
    let mut size = 0;
    for x in -10001..(max_x + 10001) {
        for y in -10001..(max_y + 10001) {
            let total_distance: i32 = input
                .iter()
                .map(|(a, b)| (x - a).abs() + (y - b).abs())
                .sum();
            if total_distance < 10000 {
                size += 1;
            }
        }
    }
    size
}

#[aoc_generator(day7)]
pub fn input_generator_day7(input: &str) -> Vec<(HashSet<char>, HashMap<char, String>)> {
    let mut map = HashMap::new();
    let mut set = HashSet::new();
    for x in input.lines() {
        let chars: Vec<char> = x.chars().collect();
        let parent = chars[5];
        let child = chars[36];
        set.insert(parent);
        set.insert(child);
        let entry = map.entry(child).or_insert(String::new());
        entry.push(parent);
    }
    vec![(set, map)]
}

#[aoc(day7, part1)]
pub fn solve_day7_part1(input: &[(HashSet<char>, HashMap<char, String>)]) -> String {
    let (set, map) = &input[0];
    let steps = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let mut finished = String::new();
    'a: while finished.len() != set.len() {
        for step in steps.chars() {
            if !set.contains(&step) {
                continue;
            }
            if finished.chars().any(|x| x == step) {
                continue;
            }
            if let Some(parents) = map.get(&step) {
                if parents.chars().all(|x| finished.chars().any(|y| x == y)) {
                    finished.push(step);
                    continue 'a;
                }
            } else {
                finished.push(step);
                continue 'a;
            }
        }
    }
    finished
}

#[aoc(day7, part2)]
pub fn solve_day7_part2(input: &[(HashSet<char>, HashMap<char, String>)]) -> usize {
    let (set, map) = &input[0];
    let steps = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let mut finished = String::new();
    let mut elves: Vec<(char, usize)> = Vec::new();
    let mut second = 0;
    'a: while finished.len() != set.len() {
        let indices_to_remove: Vec<usize> = elves
            .iter()
            .enumerate()
            .filter_map(|(i, (task, finish))| {
                if second >= *finish {
                    finished.push(*task);
                    Some(i)
                } else {
                    None
                }
            })
            .collect();
        for x in indices_to_remove.into_iter().rev() {
            elves.swap_remove(x);
        }
        if elves.len() == 5 {
            // No elves available.
            second += 1;
            continue;
        }
        for step in steps.chars() {
            if !set.contains(&step) {
                continue;
            }
            if finished.chars().any(|x| x == step) || elves.iter().any(|(x, _)| *x == step) {
                continue;
            }
            if let Some(parents) = map.get(&step) {
                if parents.chars().all(|x| finished.chars().any(|y| x == y)) {
                    elves.push((step, second + 61 + (step as usize - 'A' as usize)));
                    continue 'a;
                }
            } else {
                elves.push((step, second + 61 + (step as usize - 'A' as usize)));
                continue 'a;
            }
        }
        // Nothing can start this second.
        if finished.len() != set.len() {
            second += 1;
        }
    }
    second
}

#[aoc_generator(day8)]
pub fn input_generator_day8(input: &str) -> Vec<usize> {
    input
        .trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect()
    //vec![2, 3, 0, 3, 10, 11, 12, 1, 1, 0, 1, 99, 2, 1, 1, 2]
}

fn get_metadata_sum(mut input: &[usize]) -> (&[usize], usize) {
    let nodes = input[0];
    let metadata = input[1];
    input = &input[2..];
    let mut sum = 0;
    for _ in 0..nodes {
        let (x, y) = get_metadata_sum(input);
        input = x;
        sum += y;
    }
    sum += &input[..metadata].iter().sum();
    (&input[metadata..], sum)
}

#[aoc(day8, part1)]
pub fn solve_day8_part1(input: &[usize]) -> usize {
    let (_, sum) = get_metadata_sum(input);
    sum
}

fn make_nodes(mut input: &[usize]) -> (&[usize], usize) {
    let node_count = input[0];
    let metadata_count = input[1];
    input = &input[2..];

    let mut children = Vec::new();
    for _ in 0..node_count {
        let (x, y) = make_nodes(input);
        input = x;
        children.push(y);
    }
    let value = if node_count == 0 {
        input[..metadata_count].iter().sum()
    } else {
        let mut sum = 0;
        for m in &input[..metadata_count] {
            sum += children.get(*m - 1).unwrap_or(&0);
        }
        sum
    };
    (&input[metadata_count..], value)
}

#[aoc(day8, part2)]
pub fn solve_day8_part2(input: &[usize]) -> usize {
    let (_, root_node) = make_nodes(input);
    root_node
}

pub struct MarbleGame {
    players: usize,
    last_marble: usize,
}
#[aoc_generator(day9)]
pub fn input_generator_day9(input: &str) -> Vec<MarbleGame> {
    let re = Regex::new(r"^(\d+) players; last marble is worth (\d+) points$").unwrap();
    let cap = re.captures(input).unwrap();
    vec![MarbleGame {
        players: cap.get(1).unwrap().as_str().parse().unwrap(),
        last_marble: cap.get(2).unwrap().as_str().parse().unwrap(),
    }]
}

pub struct MarbleNode {
    left: usize,
    right: usize,
    value: usize,
}

fn solve_day9(players: usize, last_marble: usize) -> usize {
    let mut pool: Vec<MarbleNode> = Vec::with_capacity(last_marble);
    let mut scores = vec![0usize; players];

    pool.push(MarbleNode {
        left: 0,
        right: 0,
        value: 0,
    });
    let mut current_marble = 0usize;
    for marble in 1..=last_marble {
        if marble % 23 == 0 {
            let current_player = (marble - 1) % players;
            for _ in 0..7 {
                current_marble = pool[current_marble].left;
            }
            let old_left = pool[current_marble].left;
            let old_right = pool[current_marble].right;
            let value = pool[current_marble].value;
            pool[old_left].right = old_right;
            pool[old_right].left = old_left;
            current_marble = old_right;
            scores[current_player] += marble + value;
        } else {
            let right = pool[current_marble].right;
            let old_right = pool[right].right;
            pool[old_right].left = pool.len();
            pool[right].right = pool.len();
            pool.push(MarbleNode {
                left: right,
                right: old_right,
                value: marble,
            });
            current_marble = pool.len() - 1;
        }
    }
    *scores.iter().max().unwrap()
}
#[aoc(day9, part1)]
pub fn solve_day9_part1(input: &[MarbleGame]) -> usize {
    let input = &input[0];
    solve_day9(input.players, input.last_marble)
}

#[aoc(day9, part2)]
pub fn solve_day9_part2(input: &[MarbleGame]) -> usize {
    let input = &input[0];
    solve_day9(input.players, input.last_marble * 100)
}

#[derive(Copy, Clone, Debug)]
pub struct Day10Light {
    position: (i64, i64),
    velocity: (i64, i64),
}

#[aoc_generator(day10)]
pub fn input_generator_day10(input: &str) -> Vec<Day10Light> {
    let re =
        Regex::new(r"position=<\s*(-*\d+),\s*(-*\d+)>\s*velocity=<\s*(-*\d+),\s*(-*\d+)>").unwrap();
    re.captures_iter(input)
        .map(|cap| {
            let position = (cap[1].parse().unwrap(), cap[2].parse().unwrap());
            let velocity = (cap[3].parse().unwrap(), cap[4].parse().unwrap());
            Day10Light { position, velocity }
        })
        .collect()
}

use std::fs::File;
use std::io::{BufWriter, Write};

#[aoc(day10, part1)]
pub fn solve_day10_part1(input: &[Day10Light]) -> usize {
    let mut lights: Vec<Day10Light> = input.iter().cloned().collect();
    let mut min_area = 0xfffffffffffffff;
    for i in 0..11000 {
        let min_x = lights
            .iter()
            .min_by_key(|x| x.position.0)
            .unwrap()
            .position
            .0;
        let max_x = lights
            .iter()
            .max_by_key(|x| x.position.0)
            .unwrap()
            .position
            .0;
        let min_y = lights
            .iter()
            .min_by_key(|x| x.position.1)
            .unwrap()
            .position
            .1;
        let max_y = lights
            .iter()
            .max_by_key(|x| x.position.1)
            .unwrap()
            .position
            .1;
        let w = max_x - min_x;
        let h = max_y - min_y;
        if w * h < min_area {
            min_area = w * h;
        }
        if w < 100 && h < 10 {
            println!("i: {}, w,h: {}, {}, a: {}", i, w, h, w * h);
            let mut out = BufWriter::new(File::create(&format!("out{}.ppm", i)).unwrap());
            writeln!(out, "P3\n{} {}\n255", w + 6, h + 6).unwrap();
            for y in min_y - 3..max_y + 3 {
                for x in min_x - 3..max_x + 3 {
                    if lights.iter().any(|l| (x, y) == l.position) {
                        writeln!(out, "0 0 0").unwrap();
                    } else {
                        writeln!(out, "255 255 255").unwrap();
                    }
                }
            }
        }
        for l in lights.iter_mut() {
            l.position.0 += l.velocity.0;
            l.position.1 += l.velocity.1;
        }
    }
    0
}

#[aoc_generator(day11)]
pub fn input_generator_day11(input: &str) -> Vec<usize> {
    vec![input.trim().parse().unwrap()]
}

fn get_power_level(input: usize, x: usize, y: usize) -> i64 {
    let rack_id = x + 10;
    let power_level = y * rack_id;
    let power_level = power_level + input;
    let power_level = power_level * rack_id;
    let power_level = if power_level < 100 {
        0
    } else {
        (power_level / 100) % 10
    };
    power_level as i64 - 5
}

#[test]
fn power_level() {
    assert_eq!(get_power_level(8, 3, 5), 4);
    assert_eq!(get_power_level(57, 122, 79), -5);
    assert_eq!(get_power_level(39, 217, 196), 0);
    assert_eq!(get_power_level(71, 101, 153), 4);
}

#[aoc(day11, part1)]
pub fn solve_day11_part1(input: &[usize]) -> String {
    let input = input[0];
    let mut grid: Vec<Vec<i64>> = Vec::with_capacity(300);
    for y in 1..=300 {
        let mut row: Vec<i64> = Vec::with_capacity(300);
        for x in 1..=300 {
            row.push(get_power_level(input, x, y));
        }
        grid.push(row);
    }
    let mut totals: HashMap<(usize, usize), i64> = HashMap::new();
    for y in 0..297 {
        for x in 0..297 {
            let y0 = grid[y + 0][x + 0] + grid[y + 0][x + 1] + grid[y + 0][x + 2];
            let y1 = grid[y + 1][x + 0] + grid[y + 1][x + 1] + grid[y + 1][x + 2];
            let y2 = grid[y + 2][x + 0] + grid[y + 2][x + 1] + grid[y + 2][x + 2];
            totals.insert((x, y), y0 + y1 + y2);
        }
    }
    let (x, y) = *totals.iter().max_by_key(|(_, ref y)| *y).unwrap().0;
    format!("{}, {}", x + 1, y + 1)
}

#[aoc(day11, part2)]
pub fn solve_day11_part2(input: &[usize]) -> String {
    let input = input[0];
    let mut grid: Vec<Vec<i64>> = Vec::with_capacity(300);
    for y in 1..=300 {
        let mut row: Vec<i64> = Vec::with_capacity(300);
        for x in 1..=300 {
            row.push(get_power_level(input, x, y));
        }
        grid.push(row);
    }
    let mut totals: HashMap<(usize, usize, usize), i64> = HashMap::new();
    for s in 1..=300 {
        for y in 0..(300 - s) {
            for x in 0..(300 - s) {
                let entry = totals.entry((x, y, s)).or_insert(0);
                for i in 0..s {
                    for j in 0..s {
                        *entry += grid[y + i][x + j]
                    }
                }
            }
        }
    }
    let (x, y, s) = *totals.iter().max_by_key(|(_, ref y)| *y).unwrap().0;
    format!("{},{},{}", x + 1, y + 1, s)
}

#[derive(Debug)]
pub struct PlantStuff {
    initial: String,
    rules: HashMap<String, char>,
}

impl AsRef<PlantStuff> for PlantStuff {
    fn as_ref(&self) -> &PlantStuff {
        self
    }
}

#[aoc_generator(day12)]
pub fn input_generator_day12(input: &str) -> PlantStuff {
    let mut lines = input.lines();
    let initial = lines.next().unwrap()[15..].trim().to_string();
    // blank line
    lines.next();
    let mut rules = HashMap::new();
    for l in lines {
        let left = l[..6].trim().to_string();
        let right = l.trim().chars().last().unwrap();
        rules.insert(left, right);
    }
    PlantStuff { initial, rules }
}

#[aoc(day12, part1)]
pub fn solve_day12_part1(input: &PlantStuff) -> i64 {
    let rules = &input.rules;
    let mut s = format!("....{}....", input.initial);

    let mut pivot = 4i64;
    for _ in 0..5000 {
        let mut new_s = String::new();
        for x in 0..s.len() - 4 {
            let l = unsafe { s.get_unchecked(x..x + 5) };
            new_s.push(*rules.get(l).unwrap_or(&'.'));
        }
        s = format!("....{}....", new_s.trim_right_matches('.'));
        pivot += 2;
    }
    s.char_indices()
        .filter_map(|(i, c)| {
            if c == '#' {
                Some(i as i64 - pivot)
            } else {
                None
            }
        })
        .sum()
}

#[aoc(day12, part2)]
pub fn solve_day12_part2(input: &PlantStuff) -> i64 {
    let rules = &input.rules;
    let mut s = format!("....{}....", input.initial);

    let mut pivot = 4i64;
    for i in 0..50_000_000_000usize {
        let mut new_s = String::new();
        for x in 0..s.len() - 4 {
            let l = unsafe { s.get_unchecked(x..x + 5) };
            new_s.push(*rules.get(l).unwrap_or(&'.'));
        }
        if !new_s.starts_with("....") {
            let dots_to_add = 4 - (new_s.len() - new_s.trim_left_matches('.').len());
            new_s = match dots_to_add {
                4 => format!("....{}", new_s),
                3 => format!("...{}", new_s),
                2 => format!("..{}", new_s),
                1 => format!(".{}", new_s),
                _ => unreachable!(),
            };
            pivot += dots_to_add as i64 - 2;
        } else {
            pivot -= 2;
        }
        s = format!("{}....", new_s.trim_right_matches('.'));
    }
    s.char_indices()
        .filter_map(|(i, c)| {
            if c == '#' {
                Some(i as i64 - pivot)
            } else {
                None
            }
        })
        .sum()
}

#[aoc_generator(day13)]
pub fn input_generator_day13(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|l| l.chars().collect()).collect()
}

#[test]
fn test_day_13() {
    let input = r"/->-\
|   |  /----\
| /-+--+-\  |
| | |  | v  |
\-+-/  \-+--/
  \------/   ";
    assert_eq!(solve_day13_part1(&input_generator_day13(input)), "7,3");
    let input = r"/>-<\
|   |
| /<+-\
| | | v
\>+</ |
  |   ^
  \<->/";
    assert_eq!(solve_day13_part2(&input_generator_day13(input)), "6,4");
}

fn move_cars(car: &(usize, usize, char, usize), input: char) -> (usize, usize, char, usize) {
    let (mut x, mut y, mut c, mut counter) = *car;
    match (c, input, counter) {
        (_, ' ', _) => unimplemented!(),
        ('<', '/', _) | ('>', '\\', _) | ('<', '+', 0) | ('>', '+', 2) => {
            y += 1;
            c = 'v';
        }
        ('<', '\\', _) | ('>', '/', _) | ('<', '+', 2) | ('>', '+', 0) => {
            y -= 1;
            c = '^';
        }
        ('v', '/', _) | ('^', '\\', _) | ('v', '+', 2) | ('^', '+', 0) => {
            x -= 1;
            c = '<';
        }
        ('v', '\\', _) | ('^', '/', _) | ('v', '+', 0) | ('^', '+', 2) => {
            x += 1;
            c = '>';
        }
        ('<', _, _) => x -= 1,
        ('>', _, _) => x += 1,
        ('v', _, _) => y += 1,
        ('^', _, _) => y -= 1,
        _ => unimplemented!(),
    }
    if input == '+' {
        counter = (counter + 1) % 3
    }
    (x, y, c, counter)
}

#[aoc(day13, part1)]
pub fn solve_day13_part1(input: &[Vec<char>]) -> String {
    let mut cars: Vec<(usize, usize, char, usize)> = Vec::new();
    for (y, r) in input.iter().enumerate() {
        for (x, c) in r.iter().enumerate() {
            match c {
                '>' | '<' | '^' | 'v' => cars.push((x, y, *c, 0)),
                _ => {}
            }
        }
    }
    loop {
        let mut new_cars = Vec::new();
        {
            cars.sort_by_key(|(x, y, _, _)| (*x, *y));
            let mut list = cars.as_slice();
            while let Some((h, t)) = list.split_first() {
                let car = move_cars(h, input[h.1][h.0]);
                if t.iter()
                    .chain(&new_cars)
                    .any(|(x, y, _, _)| *x == car.0 && *y == car.1)
                {
                    return format!("{},{}", car.0, car.1);
                }
                new_cars.push(car);
                list = t;
            }
        }
        cars = new_cars;
    }
}

#[aoc(day13, part2)]
pub fn solve_day13_part2(input: &[Vec<char>]) -> String {
    let mut cars: Vec<(usize, usize, char, usize)> = Vec::new();
    for (y, r) in input.iter().enumerate() {
        for (x, c) in r.iter().enumerate() {
            match c {
                '>' | '<' | '^' | 'v' => cars.push((x, y, *c, 0)),
                _ => {}
            }
        }
    }
    loop {
        let mut new_cars = Vec::new();
        let mut to_remove = Vec::new();
        {
            cars.sort_by_key(|(x, y, _, _)| (*x, *y));
            let mut list = cars.as_slice();
            while let Some((h, t)) = list.split_first() {
                if to_remove.iter().any(|(x, y)| *x == h.0 && *y == h.1) {
                    to_remove.retain(|(x, y)| *x != h.0 || *y != h.1);
                    list = t;
                    continue;
                }
                let car = move_cars(h, input[h.1][h.0]);
                if t.iter()
                    .chain(&new_cars)
                    .any(|(x, y, _, _)| *x == car.0 && *y == car.1)
                {
                    let old_len = new_cars.len();
                    new_cars.retain(|(x, y, _, _)| *x != car.0 || *y != car.1);
                    if old_len == new_cars.len() {
                        to_remove.push((car.0, car.1));
                    }
                } else {
                    new_cars.push(car);
                }
                list = t;
            }
        }
        cars = new_cars;
        if cars.len() == 1 {
            return format!("{},{}", cars[0].0, cars[0].1);
        }
    }
}

#[aoc_generator(day14)]
pub fn input_generator_day14(input: &str) -> String {
    input.trim().to_string()
}

#[test]
fn test_day_14() {
    assert_eq!(solve_day14_part1(&input_generator_day14("9")), "5158916779");
    assert_eq!(solve_day14_part1(&input_generator_day14("5")), "0124515891");
    assert_eq!(
        solve_day14_part1(&input_generator_day14("18")),
        "9251071085"
    );
    assert_eq!(
        solve_day14_part1(&input_generator_day14("2018")),
        "5941429882"
    );
    assert_eq!(solve_day14_part2(&input_generator_day14("01245")), "5");
    assert_eq!(solve_day14_part2(&input_generator_day14("51589")), "9");
    assert_eq!(solve_day14_part2(&input_generator_day14("92510")), "18");
    assert_eq!(solve_day14_part2(&input_generator_day14("59414")), "2018");
}

#[aoc(day14, part1)]
pub fn solve_day14_part1(input: &str) -> String {
    let input: usize = input.parse().unwrap();
    let mut result = String::new();
    let mut elves = [0usize, 1usize];
    let mut v = vec![3u8, 7u8];
    while v.len() < input + 20 {
        let recipes = [v[elves[0]], v[elves[1]]];
        let new_recipe = recipes.iter().sum();
        let digits = if new_recipe < 10 {
            vec![new_recipe]
        } else {
            vec![new_recipe / 10, new_recipe % 10]
        };
        for d in &digits {
            v.push(*d);
            if v.len() > input && v.len() <= input + 10 {
                result += &d.to_string();
            }
        }
        for (x, y) in elves.iter_mut().zip(&recipes) {
            *x = (*x + *y as usize + 1) % v.len();
        }
    }
    result
}

#[aoc(day14, part2)]
pub fn solve_day14_part2(input: &str) -> String {
    let mut result = "37".to_string();
    let mut elves = [0usize, 1usize];
    let mut v = vec![3u8, 7u8];
    loop {
        let recipes = [v[elves[0]], v[elves[1]]];
        let new_recipe = recipes.iter().sum();
        let digits = if new_recipe < 10 {
            vec![new_recipe]
        } else {
            vec![new_recipe / 10, new_recipe % 10]
        };
        for d in &digits {
            v.push(*d);
            result += &d.to_string();
            if result.ends_with(&input) {
                return format!("{}", result.len() - input.len());
            }
        }
        for (x, y) in elves.iter_mut().zip(&recipes) {
            *x = (*x + *y as usize + 1) % v.len();
        }
    }
}

aoc_lib! { year = 2018 }
