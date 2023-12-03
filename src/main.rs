use std::{fs, io};
use std::cmp::{max};

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
    dbg!(day3()?);
    Ok(())
}
