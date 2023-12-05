use std::{fs, io};
use std::cmp::{max};

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
    let mut min_loc = u64::MAX;
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
    dbg!(day5()?);
    Ok(())
}
