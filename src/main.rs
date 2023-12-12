use itertools::Itertools;
use num::{abs, integer};
use std::cmp::{max, Ordering};
use std::collections::{HashMap, VecDeque};
use std::time::Instant;
use std::{fs, io};

fn day12() -> io::Result<(u64, u64)> {
    let data = fs::read_to_string("data/day12.in").unwrap();
    let mut p1 = 0;
    let mut p2 = 0;

    fn is_correct(data: &str, nums: &Vec<u32>) -> bool {
        let mut i = 0;
        let mut j = 0;
        while i < data.len() {
            if data.chars().nth(i).unwrap() == '.' {
                i += 1;
                continue;
            }
            if data.chars().nth(i).unwrap() == '?' {
                return false;
            }
            if data.chars().nth(i).unwrap() == '#' {
                if j == nums.len() {
                    return false;
                }
                let mut count = 0;
                while i < data.len() && data.chars().nth(i).unwrap() == '#' {
                    count += 1;
                    i += 1;
                }
                if count != nums[j] || (i != data.len() && data.chars().nth(i).unwrap() != '.') {
                    return false;
                } else {
                    j += 1;
                }
            }
        }
        if j == nums.len() {
            return true;
        }
        false
    }
    fn backtrack(data: &str, nums: &Vec<u32>, arrangements: u32, i: usize) -> u64 {
        if i == data.len() {
            return if is_correct(data, &nums) { 1 } else { 0 };
        }
        if data.chars().nth(i).unwrap() == '.' || data.chars().nth(i).unwrap() == '#' {
            return backtrack(data, nums, arrangements, i + 1);
        }
        if data.chars().nth(i).unwrap() == '?' {
            let mut new_data = data.chars().collect::<String>();
            new_data.replace_range(i..i + 1, "#");
            let danger = backtrack(new_data.as_str(), nums, arrangements, i + 1);
            new_data.replace_range(i..i + 1, ".");
            let safe = backtrack(new_data.as_str(), nums, arrangements, i + 1);
            return danger + safe;
        }
        0
    };
    let mut dp = HashMap::new();
    fn f(
        data: &str,
        nums: &Vec<u32>,
        dp: &mut HashMap<(usize, usize, usize), u64>,
        i: usize,
        ni: usize,
        len: usize,
    ) -> u64 {
        let key = &(i, ni, len);
        if dp.contains_key(key) {
            return dp[key];
        }
        if i == data.len() {
            if ni == nums.len() && len == 0 {
                return 1;
            } else if ni == nums.len() - 1 && nums[ni] == len as u32 {
                return 1;
            }
            return 0;
        }
        let mut ans = 0;
        for c in ['.', '#'] {
            if data.chars().nth(i).unwrap() == c || data.chars().nth(i).unwrap() == '?' {
                if c == '.' && len == 0 {
                    ans += f(data, nums, dp, i + 1, ni, 0);
                } else if c == '.' && len > 0 && ni < nums.len() && nums[ni] == len as u32 {
                    ans += f(data, nums, dp, i + 1, ni + 1, 0);
                } else if c == '#' {
                    ans += f(data, nums, dp, i + 1, ni, len + 1);
                }
            }
        }
        dp.insert(*key, ans);
        return ans;
    }
    for line in data.lines() {
        let mut spring = line.split_whitespace().collect::<Vec<_>>()[0].to_string();
        let mut nums = line.split_whitespace().collect::<Vec<_>>()[1]
            .split(',')
            .map(|x| x.parse::<u32>().unwrap())
            .collect::<Vec<u32>>();

        p1 += f(&spring, &nums, &mut dp, 0, 0, 0);
        dp.clear();
        spring.push('?');
        let mut new_spring = spring.repeat(5);
        new_spring.remove(new_spring.len() - 1);
        nums = nums.repeat(5);
        println!("{} {:?}", new_spring, nums);
        p2 += f(new_spring.as_str(), &nums, &mut dp, 0, 0, 0);
        dp.clear();
    }
    Ok((p1, p2))
}
fn day11() -> io::Result<(i32, u64)> {
    let data = fs::read_to_string("data/day11.in").unwrap();
    let mut mat = vec![vec!['.'; 1000]; 1000];
    let mut lines = 0;
    let mut cols = 0;
    for (i, line) in data.lines().enumerate() {
        mat[i] = line.chars().collect();
        cols = max(cols, line.chars().collect::<Vec<char>>().len());
        lines += 1;
    }
    // do lines
    let mut empty_lines = Vec::new();
    for i in 0..lines {
        let mut is_empty = true;
        for j in 0..cols {
            if mat[i][j] != '.' {
                is_empty = false;
            }
        }
        if is_empty {
            empty_lines.push(i);
        }
    }
    // do cols
    let mut empty_cols = Vec::new();
    for j in 0..cols {
        let mut is_empty = true;
        for i in 0..lines {
            if mat[i][j] != '.' {
                is_empty = false;
            }
        }
        if is_empty {
            empty_cols.push(j);
        }
    }
    let mut galaxies = Vec::new();
    for i in 0..lines {
        for j in 0..cols {
            if mat[i][j] == '#' {
                galaxies.push((i as i32, j as i32));
            }
        }
    }
    let mut p1 = 0;
    let mut p2: u64 = 0;
    for i in 0..galaxies.len() {
        for j in i + 1..galaxies.len() {
            let mut dist_p1 =
                abs(galaxies[j].0 - galaxies[i].0) + abs(galaxies[j].1 - galaxies[i].1);
            let mut dist_p2 = dist_p1 as u64;
            let (up, down) = if galaxies[i].0 < galaxies[j].0 {
                (galaxies[i].0, galaxies[j].0)
            } else {
                (galaxies[j].0, galaxies[i].0)
            };
            let upsize = 999999;
            for k in up + 1..down {
                if empty_lines.contains(&(k as usize)) {
                    dist_p1 += 1;
                    dist_p2 += upsize;
                }
            }
            let (left, right) = if galaxies[i].1 < galaxies[j].1 {
                (galaxies[i].1, galaxies[j].1)
            } else {
                (galaxies[j].1, galaxies[i].1)
            };
            for k in left + 1..right {
                if empty_cols.contains(&(k as usize)) {
                    dist_p1 += 1;
                    dist_p2 += upsize;
                }
            }
            p1 += dist_p1;
            p2 += dist_p2;
        }
    }
    Ok((p1 as i32, p2))
}
fn day10() -> io::Result<(i32, i32)> {
    let data = fs::read_to_string("data/day10.in").unwrap();
    let dirs: Vec<(i32, i32)> = vec![(1, 0), (0, 1), (-1, 0), (0, -1)];
    let mut mat = vec![vec!['.'; 1000]; 1000];
    let mut dist = vec![vec![u32::MAX; 1000]; 1000];
    let mut lines = 0;
    let mut cols = 0;
    for (i, line) in data.lines().enumerate() {
        mat[i] = line.chars().collect();
        cols = max(cols, line.chars().collect::<Vec<char>>().len());
        lines += 1;
    }
    fn connected(start: (usize, usize), end: (usize, usize), mat: &Vec<Vec<char>>) -> bool {
        let mut res = false;
        if (start.0 == end.0) {
            let (left, right) = if start.1 < end.1 {
                (start, end)
            } else {
                (end, start)
            };
            if ['-', 'L', 'F', 'S'].contains(&mat[left.0][left.1])
                && ['-', 'J', '7', 'S'].contains(&mat[right.0][right.1])
            {
                res = true;
            }
        } else if (start.1 == end.1) {
            let (bottom, top) = if start.0 > end.0 {
                (start, end)
            } else {
                (end, start)
            };
            if ['|', '7', 'F', 'S'].contains(&mat[top.0][top.1])
                && ['|', 'L', 'J', 'S'].contains(&mat[bottom.0][bottom.1])
            {
                res = true;
            }
        }
        println!(
            "start({:?}, {:?}): {:?}, end({:?}, {:?}): {:?} -> {:?}",
            start.0, start.1, mat[start.0][start.1], end.0, end.1, mat[end.0][end.1], res
        );
        return res;
    }
    let mut node = (0, 0);
    for i in 0..lines {
        for j in 0..cols {
            if mat[i][j] == 'S' {
                node = (i, j);
            }
        }
    }

    dist[node.0][node.1] = 0;
    let mut queue = VecDeque::new();
    let mut max_dist = 0;
    queue.push_back((node.0, node.1));
    while !queue.is_empty() {
        node = queue.pop_front().unwrap();
        for k in 0..dirs.len() {
            let (x, y) = (
                (node.0 as i32 + dirs[k].0) as usize,
                (node.1 as i32 + dirs[k].1) as usize,
            );
            if x < 0 || x >= lines || y < 0 || y >= cols {
                continue;
            }
            if mat[x][y] == '.' || dist[x][y] != u32::MAX {
                continue;
            }
            if connected((node.0, node.1), (x, y), &mat) {
                dist[x][y] = dist[node.0][node.1] + 1;
                if dist[x][y] > max_dist {
                    max_dist = dist[x][y];
                }
                queue.push_back((x, y));
            }
        }
    }
    for i in 0..lines {
        for j in 0..cols {
            if dist[i][j] == u32::MAX {
                mat[i][j] = '.';
            }
            print!("{}", mat[i][j]);
        }
        println!();
    }

    let mut inside = 0;
    let mut parity = 0;
    let mut start_pattern = '~';
    for i in 0..lines {
        parity = 0;
        for j in 0..cols {
            if mat[i][j] == '-' {
                continue;
            }
            if mat[i][j] == 'F' || mat[i][j] == 'L' {
                start_pattern = mat[i][j];
                continue;
            }
            if start_pattern != '~' {
                if start_pattern == 'F' && mat[i][j] == 'J' {
                    parity += 1;
                } else if start_pattern == 'L' && mat[i][j] == '7' {
                    parity += 1;
                }
                start_pattern = '~';
                continue;
            }
            if mat[i][j] == '|' {
                parity += 1;
            }
            if mat[i][j] == '.' && parity % 2 == 1 {
                inside += 1;
            }
        }
    }
    // for i in 0..lines {
    //     for j in 0..cols {
    //         print!("{:?} ", dist[i][j]);
    //     }
    //     println!();
    // }
    Ok((max_dist as i32, inside))
}

fn day9() -> io::Result<(i32, i32)> {
    let data = fs::read_to_string("data/day9.in").unwrap();
    let values = data
        .lines()
        .map(|line| {
            line.split(' ')
                .map(|x| x.parse::<i32>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    fn triangles(mut nums: Vec<i32>) -> (i32, i32) {
        if nums.iter().all(|&x| x == 0) {
            nums.extend([0, 0]);
            return (0, 0);
        }
        let (left, right) = triangles(nums.windows(2).map(|w| w[1] - w[0]).collect_vec());
        nums.push(nums[nums.len() - 1] + right);
        nums.insert(0, nums[0] - left);
        (nums[0], nums[nums.len() - 1])
    }
    let (p2, p1) = values
        .iter()
        .map(|v| triangles(v.clone()))
        .fold((0, 0), |(s1, s2), (t1, t2)| (s1 + t1, s2 + t2));
    Ok((p1, p2))
}
fn day8() -> io::Result<(usize, usize)> {
    let data = fs::read_to_string("data/day8.in").unwrap();
    let (mut p1, mut p2) = (0, 0);
    let mut graph: HashMap<String, (String, String)> = HashMap::new();
    let moves = data.lines().next().unwrap();
    for line in data.lines().skip(2) {
        let (origin, left, right) = line
            .trim_end()
            .split([' ', '=', '(', ',', ')'])
            .filter(|s| !s.is_empty())
            .collect_tuple()
            .unwrap();
        graph.insert(origin.to_string(), (left.to_string(), right.to_string()));
    }
    let mut current = "AAA";
    if graph.contains_key(current) {
        while current != "ZZZ" {
            let dir = moves.chars().nth(p1 % moves.len()).unwrap();
            current = if dir == 'L' {
                graph[current].0.as_str()
            } else {
                graph[current].1.as_str()
            };
            p1 += 1;
        }
    }

    let mut durations = Vec::new();
    for (origin, _) in graph.iter().filter(|(k, _)| k.ends_with("A")) {
        // println!("i: {:?}, crt: {:?}", i, crt);
        let mut node = origin;
        let mut num_moves = 0;
        while !node.ends_with("Z") {
            let dir = moves.chars().nth(num_moves % moves.len()).unwrap();
            node = if dir == 'L' {
                &graph[node].0
            } else {
                &graph[node].1
            };
            num_moves += 1;
        }
        durations.push(num_moves);
    }

    p2 = durations
        .iter()
        .cloned()
        .fold(1, |acc, num| integer::lcm(acc, num));
    Ok((p1, p2))
}
fn day7() -> io::Result<(usize, usize)> {
    let data = fs::read_to_string("data/day7.in").unwrap();
    let (mut p1, mut p2) = (0, 0);
    let mut cards = data
        .lines()
        .flat_map(|line| line.split_whitespace())
        .collect::<Vec<&str>>()
        .chunks(2)
        .map(|ch| (ch[0], ch[1].parse::<u32>().unwrap()))
        .collect::<Vec<(&str, u32)>>();
    let get_level = |card: &str| -> u32 {
        let mut freq = HashMap::new();
        card.chars().for_each(|c| *freq.entry(c).or_insert(0) += 1);
        let js = *freq.get(&'J').unwrap_or(&0);
        let mut counts = freq
            .iter()
            .filter(|&(&key, _)| key != 'J')
            .map(|(_, &count)| count)
            .collect::<Vec<i32>>();
        counts.sort_unstable();
        match (counts.as_slice(), js) {
            ([5], 0) | ([4], 1) | ([3], 2) | ([2], 3) | ([1], 4) | ([], 5) => 6, // five of a kind
            ([1, 4], 0) | ([1, 3], 1) | ([1, 2], 2) | ([1, 1], 3) => 5,          // four of a kind
            ([2, 3], 0) | ([2, 2], 1) => 4,                                      // FH
            ([1, 1, 3], 0) | ([1, 1, 2], 1) | ([1, 1, 1], 2) => 3,               // three of a kind
            ([1, 2, 2], 0) => 2,                                                 // two pair
            ([1, 1, 1, 2], 0) | ([1, 1, 1, 1], 1) => 1,                          // one pair
            _ => 0,
        }
    };
    let card_to_num = |card: &char| -> u32 {
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
        let a_level = get_level(a.0);
        let b_level = get_level(b.0);
        if a_level == b_level {
            for (a, b) in a.0.chars().zip(b.0.chars()) {
                if card_to_num(&a) > card_to_num(&b) {
                    return Ordering::Greater;
                } else if card_to_num(&a) < card_to_num(&b) {
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
    let times = data
        .lines()
        .next()
        .unwrap_or("")
        .split_whitespace()
        .filter(|&s| s.chars().all(|c| c.is_ascii_digit()))
        .map(|s| s.parse::<f32>().unwrap())
        .collect::<Vec<f32>>();
    let distances = data
        .lines()
        .nth(1)
        .unwrap_or("")
        .split_whitespace()
        .filter(|&s| s.chars().all(|c| c.is_ascii_digit()))
        .map(|s| s.parse::<f32>().unwrap())
        .collect::<Vec<f32>>();
    p1 = times
        .iter()
        .zip(distances.iter())
        .map(|(&t, d)| {
            ((t + (t.powf(2.0) - 4.0 * d).sqrt()) / 2.0).ceil()
                - ((t - (t.powf(2.0) - 4.0 * d).sqrt()) / 2.0).floor()
                - 1.0
        })
        .product::<f32>() as u32;
    let newtime = data
        .lines()
        .next()
        .unwrap_or("")
        .split_whitespace()
        .filter(|&s| s.chars().all(|c| c.is_ascii_digit()))
        .collect::<Vec<&str>>()
        .join("")
        .parse::<f64>()
        .unwrap();
    let newdist = data
        .lines()
        .nth(1)
        .unwrap_or("")
        .split_whitespace()
        .filter(|&s| s.chars().all(|c| c.is_ascii_digit()))
        .collect::<Vec<&str>>()
        .join("")
        .parse::<f64>()
        .unwrap();
    let disc = (newtime.powf(2.0) - 4.0 * newdist).sqrt();
    p2 = ((newtime + disc) / 2.0).ceil() - ((newtime - disc) / 2.0).floor() - 1.0;
    Ok((p1, p2))
}
fn day5() -> io::Result<(u64, u64)> {
    let data = fs::read_to_string("data/day5.in").unwrap();
    let (mut p1, mut p2) = (0, 0);
    let sections = data
        .split("\r\n")
        .filter(|&str| !str.is_empty())
        .collect::<Vec<&str>>();
    let seeds = sections[0]
        .split(':')
        .nth(1)
        .unwrap_or("")
        .trim()
        .split_whitespace()
        .map(|s| s.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();
    let mut counter = -1;
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
            ranges.push(
                sections[i]
                    .split_whitespace()
                    .map(|s| s.parse::<u64>().unwrap())
                    .collect::<Vec<u64>>(),
            );
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
    let new_seeds = seeds
        .chunks(2)
        .map(|ch| ch[0]..ch[0] + ch[1])
        .flatten()
        .collect::<Vec<u64>>();
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
    let (mut p1, mut p2) = (0, 0);
    let mut counts = vec![1; data.lines().count() + 1];
    for (i, line) in data.lines().enumerate() {
        let card = line.split(':').nth(1).unwrap_or("").trim();
        let winning = card
            .split('|')
            .next()
            .unwrap_or("")
            .trim()
            .split_whitespace()
            .filter(|s| !s.is_empty())
            .collect::<Vec<&str>>();
        let drawn = card
            .split('|')
            .nth(1)
            .unwrap_or("")
            .trim()
            .split_whitespace()
            .filter(|s| !s.is_empty())
            .collect::<Vec<&str>>();
        let count = winning
            .iter()
            .filter(|&c| drawn.iter().any(|&d| d == *c))
            .count();
        (i + 1..i + 1 + count)
            .take_while(|&j| j < data.lines().count())
            .for_each(|j| counts[j] += counts[i]);
        p1 += if count > 0 {
            u32::pow(2, (count - 1) as u32)
        } else {
            0
        };
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
            if mat[i][j] == '.' || mat[i][j].is_ascii_digit() {
                continue;
            }
            let mut nums: Vec<u32> = Vec::new();
            for k in 0..8 {
                let x = i as i32 + dirx[k];
                let mut y = j as i32 + diry[k];
                if x < 0 || x >= lines as i32 || y < 0 || y >= cols as i32 {
                    continue;
                }
                if mat[x as usize][y as usize].is_ascii_digit() && used[x as usize][y as usize] == 0
                {
                    println!("{}", mat[x as usize][y as usize]);
                    while y >= 0 && mat[x as usize][y as usize].is_ascii_digit() {
                        y -= 1;
                    }
                    println!("{}", y);
                    y += 1;
                    let mut num = 0;
                    while y < cols as i32 && mat[x as usize][y as usize].is_ascii_digit() {
                        num = num * 10 + mat[x as usize][y as usize].to_digit(10).unwrap();
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
            r = 0;
            g = 0;
            b = 0;
            let extractions = set.split(",").collect::<Vec<&str>>();
            for extraction in extractions {
                let mut j = 0;
                let mut num = 0;
                while extraction.trim().chars().nth(j).unwrap().is_ascii_digit() {
                    num = num * 10
                        + extraction
                            .trim()
                            .chars()
                            .nth(j)
                            .unwrap()
                            .to_digit(10)
                            .unwrap() as i32;
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

    Ok((p1, p2))
}
fn day1() {
    let data = fs::read_to_string("data/day1.in").unwrap();
    let mut lines = data.lines();
    let mut sum = 0;
    let digits = vec![
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    let parse_line = |line: &str| -> i32 {
        let parsing = |result: String, rev: bool| -> (i32, i32) {
            let mut aux = result;
            for (i, &digit) in digits.iter().enumerate() {
                let tofind = if !rev {
                    digit.to_string()
                } else {
                    digit.chars().rev().collect::<String>()
                };
                if let Some(pos) = aux.find(&tofind) {
                    return (pos as i32, (i + 1) as i32);
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

        let first = if first_word.0 < first_digit.0 {
            first_word
        } else {
            first_digit
        };
        let last = if last_word.0 > last_digit.0 {
            last_word
        } else {
            last_digit
        };
        first.1 * 10 + last.1
    };
    lines.for_each(|line| sum += parse_line(line));
    println!("{}", sum);
}
fn main() -> io::Result<()> {
    let now = Instant::now();
    dbg!(day12()?);
    println!("Elapsed: {:?}us", now.elapsed().as_millis());
    Ok(())
}
