use std::{fs, io};
use std::cmp::{max, Ordering};
use std::collections::HashMap;
fn gcd(mut a: usize, mut b: usize) -> usize {
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

fn lcm(a: usize, b: usize) -> usize {
    a / gcd(a, b) * b // Ensure there's no integer overflow
}

fn lcm_of_vec(numbers: &[usize]) -> usize {
    numbers.iter().cloned().fold(1, |acc, num| lcm(acc, num))
}
fn day8() -> io::Result<(usize, usize)> {
    let data = fs::read_to_string("data/day8.in").unwrap();
    let (mut p1, mut p2) = (0, 0);
    let mut graph: HashMap<String, (String, String)> = HashMap::new();
    let moves = data.lines().next().unwrap();
    println!("moves: {:?}", moves);
    for line in data.lines().skip(2) {
        let origin = line.split_whitespace().nth(0).unwrap_or("");
        let dest = line.split("=").nth(1).unwrap_or("").trim().chars().filter(|c| *c != '(' && *c != ')').collect::<String>().split(",").map(|e| e.trim().to_string()).collect::<Vec<String>>().chunks(2).map(|ch| (ch[0].to_string(), ch[1].to_string() )).collect::<Vec<(String, String)>>();
        if let Some(actual_dest) = dest.get(0) {
            graph.entry(origin.to_string()).or_insert(actual_dest.clone());
        }
        // println!("{} -> {:?}", origin, dest);
    }
    let mut current = "AAA";
    if graph.contains_key(current) {
        while current != "ZZZ" {
            for c in moves.chars() {
                // println!("current: {:?}", current);
                if current == "ZZZ" {
                    break;
                }
                if c == 'L' {
                    current = graph[current].0.as_str();
                } else {
                    current = graph[current].1.as_str();
                }
                p1 += 1;
            }
        }
    }

    let mut currents = graph.iter().filter(|(k, _)| k.ends_with("A")).map(|(k, _)| k.clone()).collect::<Vec<String>>();
    // println!("currents len: {:?}", currents.len());
    let mut durations = Vec::new();
    for (i, crt) in currents.iter().enumerate() {
        // println!("i: {:?}, crt: {:?}", i, crt);
        let mut curr = crt.clone();
        let mut num_moves = 0;
        while !curr.ends_with("Z") {
            for c in moves.chars() {
                println!("curr: {:?}", curr);
                if curr.ends_with("Z") {
                    break;
                }
                if c == 'L' {
                    curr = graph[&curr].0.clone();
                } else {
                    curr = graph[&curr].1.clone();
                }
                num_moves += 1;
            }
        }
        durations.push(num_moves);
    }

    p2 = lcm_of_vec(&durations);
    Ok((p1, p2))
}
    fn day7() -> io::Result<(usize, usize)> {
    let data = fs::read_to_string("data/day7.in").unwrap();
    let (mut p1, mut p2) = (0, 0);
    let mut cards = data.lines().flat_map(|line| line.split_whitespace()).collect::<Vec<&str>>().chunks(2).map(|ch| (ch[0], ch[1].parse::<u32>().unwrap())).collect::<Vec<(&str, u32)>>();
    let getLevel = |card: &str| -> u32 {
        let mut freq = HashMap::new();
        card.chars().for_each(|c| *freq.entry(c).or_insert(0) += 1);
        let Js = *freq.get(&'J').unwrap_or(&0);
        let mut counts = freq.iter().filter(|&(&key,_)| key != 'J').map(|(_,&count)| count).collect::<Vec<i32>>();
        counts.sort_unstable();
        match (counts.as_slice(), Js) {
            ([5],0) | ([4], 1) | ([3], 2) | ([2], 3) | ([1], 4) | ([], 5) => 6, // five of a kind
            ([1, 4], 0) | ([1, 3], 1) | ([1, 2], 2)  | ([1, 1], 3)  => 5, // four of a kind
            ([2, 3], 0) | ([2,2], 1) => 4, // FH
            ([1, 1, 3], 0) | ([1, 1, 2], 1) | ([1, 1, 1], 2) => 3, // three of a kind
            ([1, 2, 2], 0) => 2, // two pair
            ([1, 1, 1, 2], 0) | ([1, 1, 1, 1], 1) => 1, // one pair
            _ => 0,
        }
    };
    let cardToNum = |card: &char| -> u32 {
        match card {
            'A' => 14,
            'K' => 13,
            'Q' => 12,
            'J' => 1,
            'T' => 10,
            '9' => 9,
            '8' => 8,
            '7' => 7,
            '6' => 6,
            '5' => 5,
            '4' => 4,
            '3' => 3,
            '2' => 2,
            '1' => 1,
            _ => 0,
        }
    };
    cards.sort_by(|a, b| {
    let a_level = getLevel(a.0);
    let b_level = getLevel(b.0);
        if (a_level == b_level) {
            for (a, b) in a.0.chars().zip(b.0.chars()) {
               if (cardToNum(&a) > cardToNum(&b)) {
                   return Ordering::Greater;
               }
               else if (cardToNum(&a) < cardToNum(&b)) {
                   return Ordering::Less;
               }
            }
            return Ordering::Equal;
        } else {
            a_level.cmp(&b_level)
        }
    });
    for (i, card) in cards.iter().enumerate() {
       p2 += (i + 1) * card.1 as usize;
    }

    Ok((p1, p2))
}

fn day6() -> io::Result<(u32, f64)> {
    let data = fs::read_to_string("data/day6.in").unwrap();
    let (mut p1, mut p2) = (0, 0.0);
    let times = data.lines().next().unwrap_or("").split_whitespace().filter(|&s|  s.chars().all(|c| c.is_ascii_digit()) ).map(|s| s.parse::<f32>().unwrap()).collect::<Vec<f32>>();
    let distances = data.lines().nth(1).unwrap_or("").split_whitespace().filter(|&s|  s.chars().all(|c| c.is_ascii_digit()) ).map(|s| s.parse::<f32>().unwrap()).collect::<Vec<f32>>();
    p1 = times.iter().zip(distances.iter()).map(|(&t, d)| ((t + (t.powf(2.0) - 4.0 * d).sqrt()) / 2.0).ceil() - ((t - (t.powf(2.0) - 4.0 * d).sqrt()) / 2.0).floor() - 1.0).product::<f32>() as u32;
    let newtime = data.lines().next().unwrap_or("").split_whitespace().filter(|&s|  s.chars().all(|c| c.is_ascii_digit()) ).collect::<Vec<&str>>().join("").parse::<f64>().unwrap();
    let newdist = data.lines().nth(1).unwrap_or("").split_whitespace().filter(|&s|  s.chars().all(|c| c.is_ascii_digit()) ).collect::<Vec<&str>>().join("").parse::<f64>().unwrap();
    let disc = (newtime.powf(2.0) - 4.0 * newdist).sqrt();
    p2 = ((newtime + disc) / 2.0).ceil() - ((newtime - disc) / 2.0).floor() - 1.0;
    Ok((p1, p2))
}
fn day5() -> io::Result<(u64, u64)> {
    let data = fs::read_to_string("data/day5.in").unwrap();
    let (mut p1, mut p2) = (0, 0);
    let sections = data.split("\r\n").filter(|&str| !str.is_empty()).collect::<Vec<&str>>();
    let seeds = sections[0].split(':').nth(1).unwrap_or("").trim().split_whitespace().map(|s| s.parse::<u64>().unwrap()).collect::<Vec<u64>>();
    let mut counter  = -1;
    let mut maps = Vec::new();
    let mut i = 1;
    while i < sections.len() {
        if sections[i].contains("map") {
            counter += 1;
            i += 1;
            continue;
        }
        let mut ranges = Vec::new();
        while i < sections.len() && !sections[i].contains("map") {
           ranges.push(sections[i].split_whitespace().map(|s| s.parse::<u64>().unwrap()).collect::<Vec<u64>>());
            i += 1;
        }
        maps.push(ranges);
        i += 1;
    }

   let mut locations = Vec::new();
   for seed in &seeds {
       let mut current_num = *seed;
       for map in &maps {
           for range in map {
               if current_num >= range[1] && current_num <= range[1] + range[2] {
                   current_num = range[0] + (current_num - range[1]);
                   break;
               }
           }
       }
       locations.push(current_num);
   }
    i = 0;
    let new_seeds = seeds.chunks(2).map(|ch| (ch[0]..ch[0] + ch[1])).flatten().collect::<Vec<u64>>();
    // Not in mood to deal with range intersections and what not.
    let mut new_locations = Vec::new();
    for seed in new_seeds {
        let mut current_num = seed;
        for map in &maps {
            for range in map {
                if current_num >= range[1] && current_num < range[1] + range[2] {
                    current_num = range[0] + (current_num - range[1]);
                    break;
                }
            }
        }
        new_locations.push(current_num);
    }
    p1 = *locations.iter().min().unwrap();
    p2 = *new_locations.iter().min().unwrap();
    Ok((p1, p2))
}

fn day4() -> io::Result<(u32, u32)> {
    let data = fs::read_to_string("data/day4.in").unwrap();
    let (mut p1, mut p2) = (0,0);
    let mut counts = vec![1; (data.lines().count() + 1) as usize];
    for (i, line) in data.lines().enumerate() {
        let card = line.split(':').nth(1).unwrap_or("").trim();
        let winning = card.split('|').next().unwrap_or("").trim().split_whitespace().filter(|s| !s.is_empty()).collect::<Vec<&str>>();
        let drawn = card.split('|').nth(1).unwrap_or("").trim().split_whitespace().filter(|s| !s.is_empty()).collect::<Vec<&str>>();
        let count = winning.iter().filter(|&c| drawn.iter().any(|&d| d == *c)).count();
        (i+1..i+1+count).take_while(|&j| j < data.lines().count()).for_each(|j| counts[j] += counts[i]);
        p1 += if count > 0 { u32::pow(2, (count-1) as u32) } else { 0 };
        p2 += counts[i];
    }
    Ok((p1, p2))
}

fn day3() -> io::Result<(u32, u32)> {
    let data = fs::read_to_string("data/day3.in").unwrap();
    let mut p1: u32 = 0;
    let mut p2: u32 = 0;
    let dirx = [1, 0, -1, 0, 1, -1, 1, -1];
    let diry = [0, 1, 0, -1, 1, 1, -1, -1];
    let mut mat = vec![vec!['.'; 1000]; 1000];
    let mut used = vec![vec![0; 1000]; 1000];
    let mut lines = 0;
    let mut cols = 0;
    for (i, line) in data.lines().enumerate() {
        mat[i] = line.chars().collect();
        cols = max(cols, line.chars().collect::<Vec<char>>().len());
        lines += 1;
    }

    for i in 0..lines {
        for j in 0..cols {
            if mat[i][j] == '.' || mat[i][j].is_ascii_digit()  {
               continue;
            }
            let mut nums: Vec<u32> = Vec::new();
            for k in 0..8 {
                let x = i as i32 + dirx[k] as i32;
                let mut y = j as i32 + diry[k] as i32;
                if x < 0 || x >= lines as i32 || y < 0 || y >= cols as i32 {
                    continue;
                }
                if mat[x as usize][y as usize].is_ascii_digit() && used[x as usize][y as usize] == 0 {
                    println!("{}", mat[x as usize][y as usize]);
                    while (y >= 0 && mat[x as usize][y as usize].is_ascii_digit()) {
                        y -= 1;
                    }
                    println!("{}", y);
                    y += 1;
                    let mut num = 0;
                    while (y < cols as i32 && mat[x as usize][y as usize].is_ascii_digit()) {
                        num = num * 10 + mat[x as usize][y as usize].to_digit(10).unwrap() as u32;
                        used[x as usize][y as usize] = 1;
                        y += 1;
                    }
                    println!("{}", num);
                    p1 += num;
                    nums.push(num);
                }
            }
            if mat[i][j] == '*' && nums.len() == 2 {
                p2 += nums[0] * nums[1];
            }
        }
    }

    Ok((p1, p2))
}

fn day2() -> io::Result<(usize, u32)> {
    let data = fs::read_to_string("data/day2.in").unwrap();
    let mut p1 = 0;
    let mut p2: u32 = 0;
    let p1r = 12;
    let p1g = 13;
    let p1b = 14;
    for (i, line) in data.lines().enumerate() {
        let games = line.split(":").collect::<Vec<&str>>()[1];
        let sets = games.split(";").collect::<Vec<&str>>();
        let mut r = 0;
        let mut g = 0;
        let mut b = 0;
        let mut maxr = 0;
        let mut maxg = 0;
        let mut maxb = 0;
        let mut is_ok = true;
        for set in sets {
            r = 0; g = 0; b = 0;
            let extractions = set.split(",").collect::<Vec<&str>>();
            for extraction in extractions {
                let mut j = 0;
                let mut num = 0;
                while extraction.trim().chars().nth(j).unwrap().is_ascii_digit() {
                    num = num * 10 + extraction.trim().chars().nth(j).unwrap().to_digit(10).unwrap() as i32;
                    j += 1;
                }
                r += if extraction.contains("red") { num } else { 0 };
                g += if extraction.contains("green") { num } else { 0 };
                b += if extraction.contains("blue") { num } else { 0 };
            }
            if r > p1r || g > p1g || b > p1b {
                is_ok = false;
            }
            maxr = max(maxr, r as u32);
            maxg = max(maxg, g as u32);
            maxb = max(maxb, b as u32);
        }
        if is_ok {
            p1 += i + 1;
        }
        p2 += maxr * maxg * maxb;
    }

    Ok((p1,p2))
}
fn day1() {
    let data = fs::read_to_string("data/day1.in").unwrap();
    let mut lines =data.lines();
    let mut sum = 0;
    let digits = vec!["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
    let parse_line = |line: &str| -> i32 {
        let parsing = |result: String, rev: bool| -> (i32, i32) {
            let mut aux = result;
            for (i, &digit) in digits.iter().enumerate() {
                let tofind = if !rev { digit.to_string() } else { digit.chars().rev().collect::<String>() };
                if let Some(pos) = aux.find(&tofind) {
                    return (pos as i32, (i+1) as i32);
                }
            }
            (999999, -1)
        };
        let first_word = parsing(line.to_string(), false);
        let last_word = parsing(line.to_string().chars().rev().collect::<String>(), true);

        let get_digit = |l: String| -> (i32, i32) {
            for (i, c) in line.chars().enumerate() {
                if c.is_ascii_digit() {
                    return (i as i32, c as i32);
                }
            }
            (999999, -1)
        };
       let first_digit = get_digit(line.to_string());
       let last_digit = get_digit(line.to_string().chars().rev().collect::<String>());

        let first = if first_word.0 < first_digit.0 { first_word } else { first_digit };
        let last = if last_word.0 > last_digit.0 { last_word } else { last_digit };
        first.1 * 10 + last.1
    };
    lines.for_each(|line| sum += parse_line(line));
    println!("{}", sum);
}
fn main() -> io::Result<()> {
    dbg!(day8()?);
    Ok(())
}
