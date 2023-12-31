#![allow(dead_code)]
use itertools::Itertools;
use num::{abs, integer};
use std::cmp::{max, Ordering, Reverse};
use std::collections::{BinaryHeap, HashMap, VecDeque};
use std::time::Instant;
use std::{fs, io};

fn day25() -> io::Result<(u64, u64)> {
    let data = fs::read_to_string("data/day25.in").unwrap();
    let (mut p1, mut p2) = (0, 0);
    let mut G: HashMap<String, Vec<String>> = HashMap::new();
    for line in data.lines() {
        let origin = line.split(':').nth(0).unwrap();
        let dest = line
            .split(':')
            .nth(1)
            .unwrap()
            .split_whitespace()
            .filter(|x| !x.is_empty())
            .collect::<Vec<_>>();
        for v in dest {
            let dests = G.entry(origin.to_string()).or_insert(Vec::new());
            if !dests.contains(&v.to_string()) {
                dests.push(v.to_string());
            }
            let other = G.entry(v.to_string()).or_insert(Vec::new());
            if !other.contains(&origin.to_string()) {
                other.push(origin.to_string());
            }
        }
    }
    println!("{:?}", G);
    Ok((p1, p2))
}

fn day24() -> io::Result<(u64, u64)> {
    let data = fs::read_to_string("data/day24.in").unwrap();
    let (mut p1, mut p2) = (0, 0);
    let hails = data
        .lines()
        .map(|line| {
            line.split([' ', ',', '@'])
                .filter(|s| !s.is_empty())
                .map(|s| s.parse::<i64>().unwrap())
                .collect_tuple::<(i64, i64, i64, i64, i64, i64)>()
                .unwrap()
        })
        .collect_vec();
    println!("{:?}", hails);
    const LOWER_BOUNDARY: i64 = 200_000_000_000_000;
    // const LOWER_BOUNDARY: i64 = 7;
    const UPPER_BOUNDARY: i64 = 400_000_000_000_000;
    // const UPPER_BOUNDARY: i64 = 22;

    for i in 0..hails.len() {
        for j in i + 1..hails.len() {
            let mi = hails[i].4 as f64 / hails[i].3 as f64;
            let mj = hails[j].4 as f64 / hails[j].3 as f64;

            let fi = mi * -hails[i].0 as f64 + hails[i].1 as f64;
            let fj = mj * -hails[j].0 as f64 + hails[j].1 as f64;

            if mi == mj && fi != fj {
                continue;
            } else {
                let ix = (fj - fi) / (mi - mj);
                let iy = mi * ix + fi;
                if (ix > hails[i].0 as f64 && hails[i].3 < 0)
                    || (ix > hails[j].0 as f64 && hails[j].3 < 0)
                    || (ix < hails[i].0 as f64 && hails[i].3 > 0)
                    || (ix < hails[j].0 as f64 && hails[j].3 > 0)
                {
                    continue;
                }
                println!("{} {} {:?}", i, j, (ix, iy));
                if ix >= LOWER_BOUNDARY as f64
                    && ix <= UPPER_BOUNDARY as f64
                    && iy >= LOWER_BOUNDARY as f64
                    && iy <= UPPER_BOUNDARY as f64
                {
                    p1 += 1;
                }
            }
        }
    }
    Ok((p1, p2))
}

fn day23() -> io::Result<(u64, u64)> {
    let data = fs::read_to_string("data/day23.in").unwrap();
    let (mut p1, mut p2) = (0, 0);
    let mut grid = HashMap::new();
    let (mut start, mut end) = ((0, 0), (0, 0));
    let (n, m) = (data.lines().count(), data.lines().next().unwrap().len());
    for (i, line) in data.lines().enumerate() {
        for j in 0..line.len() {
            grid.insert((i, j), line.chars().nth(j).unwrap());
            if (i == 0 && grid[&(i, j)] == '.') {
                start = (i, j);
            }
            if (i == data.lines().count() - 1 && grid[&(i, j)] == '.') {
                end = (i, j);
            }
        }
    }
    fn count_neighbours(
        grid: &HashMap<(usize, usize), char>,
        i: usize,
        j: usize,
        n: usize,
        m: usize,
    ) -> usize {
        let mut count = 0;
        for (di, dj) in [(0, 1), (1, 0), (0, -1), (-1, 0)] {
            if di == 0 && dj == 0 {
                continue;
            }

            let dx = (i as isize + di);
            let dy = (j as isize + dj);
            if dx < 0 || dx >= n as isize || dy < 0 || dy >= m as isize {
                continue;
            }
            let (x, y) = (dx as usize, dy as usize);
            if grid.contains_key(&(x, y)) && grid[&(x, y)] != '#' {
                count += 1;
            }
        }
        count
    }
    let mut skip_list = HashMap::new();
    let mut all_tunnels = Vec::new();
    let mut in_tunnels = Vec::new();
    let mut S = VecDeque::new();
    let mut seen = HashMap::new();
    S.push_back(start);
    while let Some((i, j)) = S.pop_front() {
        if seen.contains_key(&(i, j)) {
            continue;
        }
        seen.insert((i, j), true);
        // look down and right
        for d in [(1isize, 0isize), (0, 1), (-1, 0), (0, -1)] {
            let (mut k, mut l) = (i as isize + d.0, j as isize + d.1);
            let mut distance: usize = 1;
            let mut prev = (i, j);
            let mut path_nodes = Vec::new();
            while k >= 0 && k < n as isize && l >= 0 && l < m as isize {
                if grid[&(k as usize, l as usize)] == '#'
                    || count_neighbours(&grid, k as usize, l as usize, n, m) != 2
                    || in_tunnels.contains(&(k as usize, l as usize))
                {
                    break;
                }
                path_nodes.push((k as usize, l as usize));
                let mut next = (0, 0);
                for (di, dj) in [(1, 0), (0, 1), (-1, 0), (0, -1)] {
                    let (x, y) = (k as isize + di, l as isize + dj);
                    if x < 0 || x >= n as isize || y < 0 || y >= m as isize {
                        continue;
                    }
                    let (x, y) = (x as usize, y as usize);
                    if grid[&(x, y)] != '#' && prev != (x, y) {
                        next = (x, y);
                        prev = (k as usize, l as usize);
                        distance += 1;
                        break;
                    }
                }
                (k, l) = (next.0 as isize, next.1 as isize);
            }
            if (distance > 1) {
                skip_list.insert(((i, j), d), ((k as usize, l as usize), distance));
                skip_list.insert(
                    (
                        (k as usize, l as usize),
                        (prev.0 as isize - k, prev.1 as isize - l),
                    ),
                    ((i, j), distance),
                );
                all_tunnels.push(path_nodes.clone());
                for p in &path_nodes[1..] {
                    in_tunnels.push(*p);
                }
                S.push_back((k as usize, l as usize));
            }
        }
    }
    for p in all_tunnels {
        for n in p {
            grid.insert((n.0, n.1), '0');
        }
    }
    // println!("{:?}", skip_list[&((5, 3), (1, 0))]);
    for i in 0..n {
        for j in 0..m {
            print!("{}", grid[&(i, j)]);
        }
        println!();
    }
    println!("{:?}", skip_list);

    fn DFS(
        grid: &HashMap<(usize, usize), char>,
        start: (usize, usize),
        end: (usize, usize),
        n: usize,
        m: usize,
        part1: bool,
        skip_list: &HashMap<((usize, usize), (isize, isize)), ((usize, usize), usize)>,
    ) -> u64 {
        let mut ans = 0;
        let mut S = VecDeque::new();
        let mut seen = HashMap::new();
        let mut path = vec![start];
        seen.insert(start, true);
        S.push_back((start, 0, path, seen));
        while let Some(item) = S.pop_front() {
            let (node, dist, path, current_seen) = item;
            // println!("Visiting node: {:?} - {}", node, dist);
            if node == end {
                ans = ans.max(dist);
                println!("Found with dist: {}", dist);
                // for i in 0..n {
                //     for j in 0..m {
                //         if path.contains(&(i, j)) {
                //             print!("X");
                //         } else {
                //             print!("{}", grid.get(&(i, j)).unwrap());
                //         }
                //     }
                //     println!();
                // }
            }
            let mut dirs = vec![(0, 1), (1, 0), (0, -1), (-1, 0)];
            if part1 {
                if ['<', '^', 'v', '>'].contains(&grid[&(node.0, node.1)]) {
                    match grid[&(node.0, node.1)] {
                        '<' => dirs = vec![(0, -1)],
                        '^' => dirs = vec![(-1, 0)],
                        'v' => dirs = vec![(1, 0)],
                        '>' => dirs = vec![(0, 1)],
                        _ => unreachable!(),
                    }
                }
            }
            for (idx, idy) in dirs {
                let (x, y) = (node.0 as isize + idx, node.1 as isize + idy);
                if x < 0 || x >= n as isize || y < 0 || y >= m as isize {
                    continue;
                }
                let (mut ux, mut uy) = (x as usize, y as usize);
                let mut skip_dist = 1;
                if !part1 {
                    if skip_list.contains_key(&(node, (idx, idy))) {
                        ((ux, uy), skip_dist) = *skip_list.get(&(node, (idx, idy))).unwrap();
                    }
                }
                if grid[&(ux, uy)] != '#' && !current_seen.contains_key(&(ux, uy)) {
                    let mut new_seen = current_seen.clone();
                    new_seen.insert((ux, uy), true);
                    let mut new_path = path.clone();
                    new_path.extend([(ux, uy)]);
                    S.push_back(((ux, uy), dist + skip_dist as u64, new_path, new_seen));
                }
            }
        }
        return ans;
    }
    // p1 = DFS(&grid, start, end, n, m, true, &skip_list);
    p2 = DFS(&grid, start, end, n, m, false, &skip_list);
    Ok((p1, p2))
}

fn day22() -> io::Result<(u64, u64)> {
    let data = fs::read_to_string("data/day22.in").unwrap();
    let (mut p1, mut p2) = (0, 0);
    let mut bricks = Vec::new();
    for line in data.lines() {
        let coords = line
            .split([',', '~'])
            .filter(|x| !x.is_empty())
            .map(|x| x.parse::<usize>().unwrap())
            .collect::<Vec<_>>();
        bricks.push((
            (coords[0], coords[1], coords[2]),
            (coords[3], coords[4], coords[5]),
        ));
    }
    // ToDo: Do we need a more advanced sorting algorithm here?
    bricks.sort_by(|a, b| a.0 .2.cmp(&b.0 .2));

    fn dropped_brick(
        tallest: &HashMap<(usize, usize), usize>,
        x: &((usize, usize, usize), (usize, usize, usize)),
    ) -> ((usize, usize, usize), (usize, usize, usize)) {
        let mut peak: usize = 0;
        for i in x.0 .0..=x.1 .0 {
            for j in x.0 .1..=x.1 .1 {
                peak = max(peak, *tallest.get(&(i, j)).unwrap_or(&0));
            }
        }
        let dz = max(0, x.0 .2 - peak - 1);
        return ((x.0 .0, x.0 .1, x.0 .2 - dz), (x.1 .0, x.1 .1, x.1 .2 - dz));
    }
    fn drop(
        bricks: Vec<((usize, usize, usize), (usize, usize, usize))>,
    ) -> (u64, Vec<((usize, usize, usize), (usize, usize, usize))>) {
        let mut tallest = HashMap::new();
        let mut new_tower = Vec::new();
        let mut falls: u64 = 0;
        for b in bricks {
            let new_brick = dropped_brick(&tallest, &b);
            if new_brick.0 .2 != b.0 .2 {
                falls += 1
            }
            new_tower.push(new_brick);
            for x in (b.0 .0..=b.1 .0) {
                for y in (b.0 .1..=b.1 .1) {
                    tallest.insert((x, y), new_brick.1 .2);
                }
            }
        }
        return (falls, new_tower);
    }

    let (_, fallen) = drop(bricks);
    for i in 0..fallen.len() {
        let mut removed = fallen.clone();
        removed.remove(i);
        let (mut falls, _) = drop(removed);
        if falls == 0 {
            p1 += 1;
        } else {
            p2 += falls;
        }
    }
    Ok((p1, p2))
}

fn day21() -> io::Result<(u64, u64)> {
    let data = fs::read_to_string("data/day21.in").unwrap();
    let (mut p1, mut p2) = (0, 0);
    let mut grid = data
        .lines()
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect_vec();
    let (mut x, mut y) = (0, 0);
    let mut gradens_counter = 0;
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if grid[i][j] == 'S' {
                x = i;
                y = j;
            } else if grid[i][j] == '.' {
                gradens_counter += 1;
            }
        }
    }
    let mut queue = VecDeque::new();

    fn step(grid: &mut Vec<Vec<char>>, queue: &mut VecDeque<(i64, i64)>) {
        let mut step_queue = queue.clone();
        queue.clear();
        while let Some((x, y)) = step_queue.pop_front() {
            for (dx, dy) in vec![(0, 1), (1, 0), (-1, 0), (0, -1)] {
                let nx = x + dx;
                let ny = y + dy;
                let mx = grid.len() as i64;
                let my = grid[0].len() as i64;
                let cx = ((nx % mx + mx) % mx) as usize;
                let cy = ((ny % my + my) % my) as usize;
                if grid[cx][cy] == '.' {
                    if !queue.contains(&(nx, ny)) {
                        queue.push_back((nx, ny));
                    }
                }
            }
        }
    }
    grid[x][y] = '.';
    let N = grid.len() as i64;
    for k in [1] {
        // [65, 65 + N, 65 + 2 * N] {
        // for k in [50, 100, 500] {
        queue.clear();
        queue.push_back((x as i64, y as i64));
        println!("k = {}", k);
        for i in 0..k {
            step(&mut grid, &mut queue);
            if i % 100 == 0 {
                println!("{} steps", i);
            }
        }
        println!("ans = {}", queue.len());
    }

    fn f(n: u64) -> u64 {
        let (a0, a1, a2) = (3814, 33952, 94138);
        let (b0, b1, b2) = (a0, a1 - a0, a2 - a1);
        return b0 + n * b1 + (b2 - b1) * (n * (n - 1) / 2);
    }
    println!("f = {}", f(26501365 / 131));
    p1 = queue.len() as u64;
    // for i in 0..grid.len() {
    //     for j in 0..grid[i].len() {
    //         print!("{}", grid[i][j]);
    //     }
    //     println!();
    // }
    Ok((p1, p2))
}

fn day20() -> io::Result<(u64, u64)> {
    let data = fs::read_to_string("data/day20.in").unwrap();
    let (mut p1, mut p2) = (0, 0);
    let mut graph = HashMap::new();
    let mut graph_types = HashMap::new();
    let mut flipflops = HashMap::new();
    let mut conjunctions = HashMap::<&str, Vec<(&str, i32)>>::new();
    for line in data.lines() {
        let mut new_line = line.clone();
        let t = if ["%", "&"].contains(&&line[0..1]) {
            new_line = &new_line[1..];
            &line[0..1]
        } else {
            "#"
        };
        let a = new_line.split("->").next().unwrap().trim();
        let b = new_line
            .split("->")
            .nth(1)
            .unwrap()
            .split(",")
            .filter(|s| !s.is_empty())
            .map(|s| s.trim())
            .collect_vec();
        graph.insert(a, b);
        graph_types.insert(a, t);
        if t == "%" {
            flipflops.insert(a, -1);
        }
    }
    graph.insert("button", vec!["broadcaster"]);
    graph_types.insert("button", "#");
    let mut new_entries = Vec::new();
    for (n, v) in &graph {
        for w in v {
            if !graph_types.contains_key(w) {
                new_entries.push(*w);
            }
        }
    }
    for w in new_entries {
        graph_types.insert(w, "#");
        graph.insert(w, Vec::new());
    }
    for (n, t) in &graph_types {
        if *t == "&" {
            // look for all neighbours
            for (origin, dest) in &graph {
                if dest.contains(&n) {
                    conjunctions
                        .entry(n)
                        .or_insert_with(Vec::new)
                        .push((origin, -1));
                }
            }
        }
    }
    for (node, conj) in &mut conjunctions {
        conj.iter().unique().collect_vec();
    }
    println!("{:?}", graph);
    println!("{:?}", graph_types);
    println!("{:?}", flipflops);
    println!("{:?}", conjunctions);
    fn DFS<'a>(
        graph: &HashMap<&str, Vec<&'a str>>,
        graph_types: &HashMap<&str, &str>,
        flipflops: &mut HashMap<&'a str, i32>,
        conjunctions: &mut HashMap<&'a str, Vec<(&str, i32)>>,
        lcm: &mut HashMap<&'a str, i32>,
        presses: i32,
        start: &str,
    ) -> (i64, i64) {
        let mut queue = VecDeque::new();
        let (mut lows, mut highs) = (0, 0);
        queue.push_back((start, -1));
        while let Some((node, signal)) = queue.pop_front() {
            for n in &graph[node] {
                if signal == -1 {
                    lows += 1;
                } else {
                    highs += 1;
                }
                if *n == "rx" && signal == -1 {
                    return (-1, -1);
                }
                if graph_types[n] == "#" {
                    queue.push_back((n, signal));
                } else if graph_types[n] == "%" {
                    if signal == -1 {
                        let status = flipflops[n] * -1;
                        flipflops.insert(n, status);
                        queue.push_back((n, status));
                    }
                } else if graph_types[n] == "&" {
                    if let Some(conjunction) = conjunctions.get_mut(n) {
                        for c in &mut *conjunction {
                            if c.0 == node {
                                c.1 = signal;
                            }
                        }
                        if conjunction.iter().any(|(v, s)| *s == -1) {
                            queue.push_back((n, 1));
                            if ["sr", "sn", "rf", "vq"].contains(n) {
                                if !lcm.contains_key(n) {
                                    lcm.insert(n, presses);
                                }
                            }
                        } else {
                            queue.push_back((n, -1));
                        }
                    }
                }
            }
        }
        (lows, highs)
    }
    // p1 - lazy to reset
    // let (mut lows, mut highs) = (0, 0);
    // for i in 0..1000 {
    //     let (l, h) = DFS(
    //         &graph,
    //         &graph_types,
    //         &mut flipflops,
    //         &mut conjunctions,
    //         "button",
    //     );
    //     lows += l;
    //     highs += h;
    //     // println!("flips: {:?}", flipflops);
    //     // println!("conj: {:?}", conjunctions);
    // }
    // println!("{:?}", lows);
    // println!("{:?}", highs);
    // p1 = lows * highs;
    let mut lcm = HashMap::new();
    let (mut l, mut r) = (0, 0);
    while (l, r) != (-1, -1) {
        p2 += 1;
        (l, r) = DFS(
            &graph,
            &graph_types,
            &mut flipflops,
            &mut conjunctions,
            &mut lcm,
            p2 as i32,
            "button",
        );
        if lcm.len() == 4 {
            p2 = lcm
                .iter()
                .map(|(s, v)| *v as u64)
                .collect_vec()
                .iter()
                .fold(1u64, |acc, num| integer::lcm(acc, *num));
            break;
        }
    }
    Ok((p1, p2))
}

fn day19() -> io::Result<(u64, u64)> {
    let data = fs::read_to_string("data/day19.in").unwrap();
    let (mut p1, mut p2) = (0, 0);
    let (wf, pts) = data
        .split_once("\r\n\r\n")
        .map(|(a, b)| (a.lines().collect_vec(), b.lines().collect_vec()))
        .unwrap();
    let mut parts = Vec::new();
    for pt in pts {
        parts.push(
            pt.split(['{', '}', 'x', 'm', 'a', 's', '=', ','])
                .filter(|s| !s.is_empty())
                .map(|s| s.parse::<u64>().unwrap_or(0))
                .collect_tuple()
                .unwrap_or((0, 0, 0, 0)),
        );
    }
    let mut wfs = HashMap::new();
    for w in wf {
        wfs.insert(
            w.split('{').next().unwrap(),
            w.split(['{', ',', '}'])
                .skip(1)
                .filter(|s| !s.is_empty())
                .collect_vec(),
        );
    }

    fn parse_rule(rls: &Vec<&str>, p: (u64, u64, u64, u64)) -> String {
        for r in rls {
            // Endoing node
            if !r.contains(":") {
                return r.to_string();
            }
            let (cond, res) = r.split_once(":").unwrap();
            let (l, op, v) = (
                cond.as_bytes()[0],
                cond.as_bytes()[1],
                cond[2..].parse::<u64>().unwrap(),
            );
            println!("Evaluating rule {} {} {}", l, op, v);
            let part_num = match l {
                b'x' => p.0,
                b'm' => p.1,
                b'a' => p.2,
                b's' => p.3,
                _ => unreachable!(),
            };
            if op == b'<' && part_num < v {
                return res.to_string();
            } else if op == b'>' && part_num > v {
                return res.to_string();
            }
        }
        return String::new();
    }

    for p in parts {
        let mut result = String::from("in");
        while !["A", "R"].contains(&result.as_str()) {
            result = parse_rule(&wfs[result.as_str()], p);
        }
        if result == "A" {
            p1 += p.0 + p.1 + p.2 + p.3;
        }
    }

    fn solve_p2(rls: &HashMap<&str, Vec<&str>>, current: &str, p: &Vec<(u64, u64)>) -> u64 {
        let mut ans = 0;
        for r in &rls[current] {
            // Endoing node
            if *r == "A" {
                println!(
                    "{} - {:?} = {}",
                    current,
                    p,
                    (p[0].1 - p[0].0 + 1)
                        * (p[1].1 - p[1].0 + 1)
                        * (p[2].1 - p[2].0 + 1)
                        * (p[3].1 - p[3].0 + 1)
                );
                ans += (p[0].1 - p[0].0 + 1)
                    * (p[1].1 - p[1].0 + 1)
                    * (p[2].1 - p[2].0 + 1)
                    * (p[3].1 - p[3].0 + 1);
                continue;
            }
            if *r == "R" {
                return ans;
            }
            if !r.contains(":") {
                ans += solve_p2(rls, r, p);
                continue;
            }
            // Got a rule
            let (cond, res) = r.split_once(":").unwrap();
            let (l, op, mut v) = (
                cond.as_bytes()[0],
                cond.as_bytes()[1],
                cond[2..].parse::<u64>().unwrap(),
            );
            let part_num = match l {
                b'x' => 0,
                b'm' => 1,
                b'a' => 2,
                b's' => 3,
                _ => unreachable!(),
            };
            let mut new_part = p.clone();
            if op == b'<' {
                if p[part_num].0 < v && v < p[part_num].1 {
                    // left interval
                    new_part[part_num].1 = v - 1;
                    ans += solve_p2(rls, res, &new_part);
                    // right interval
                    new_part = p.clone();
                    new_part[part_num].0 = v;
                    ans += solve_p2(rls, current, &new_part);
                    return ans;
                } else if p[part_num].1 < v {
                    ans += solve_p2(rls, res, &new_part);
                    return ans;
                    continue;
                }
            } else {
                if p[part_num].0 < v && v < p[part_num].1 {
                    // left interval
                    new_part[part_num].1 = v;
                    ans += solve_p2(rls, current, &new_part);
                    // right interval
                    new_part = p.clone();
                    new_part[part_num].0 = v + 1;
                    ans += solve_p2(rls, res, &new_part);
                    return ans;
                } else if p[part_num].0 > v {
                    return solve_p2(rls, res, &new_part);
                }
            }
        }
        return ans;
    }
    wfs.insert("A", vec!["A"]);
    wfs.insert("R", vec!["R"]);
    p2 = solve_p2(
        &wfs,
        "in",
        &vec![(1, 4000), (1, 4000), (1, 4000), (1, 4000)],
    );
    // println!("{:?}", wfs);
    // println!("{:?}", parts);
    Ok((p1, p2))
}

fn day18() -> io::Result<(u64, u64)> {
    let data = fs::read_to_string("data/day18.in").unwrap();
    let (mut p1, mut p2) = (0, 0);

    let mut current = (0, 0);
    let (mut n, mut m) = (0, 0);

    let mut points = Vec::new();
    let mut grid = vec![vec![('.', ""); 100]; 100];
    points.push(((0, 0), ""));
    let mut grid_points = 0;
    grid[0][0] = ('#', "");
    for line in data.lines() {
        let dir = line.split_whitespace().nth(0).unwrap();
        let mut cnt = line
            .split_whitespace()
            .nth(1)
            .unwrap()
            .parse::<isize>()
            .unwrap();
        let color = line
            .split_whitespace()
            .nth(2)
            .unwrap()
            .trim_matches(|c| c == '(' || c == ')')
            .trim_matches('#');
        cnt = isize::from_str_radix(&color[0..color.len() - 1], 16).unwrap();
        let mut direction = match color.as_bytes()[color.len() - 1] {
            b'3' => (-cnt, 0),
            b'1' => (cnt, 0),
            b'2' => (0, -cnt),
            b'0' => (0, cnt),
            _ => unreachable!(),
        };
        current = (current.0 + direction.0, current.1 + direction.1);
        grid_points += cnt;
        points.push(((current.0, current.1), color));
    }
    let mut s = points
        .iter()
        .tuple_windows()
        .map(|(((x1, y1), _), ((x2, y2), _))| (y1 + y2) * (x2 - x1))
        .sum::<isize>();
    s = (grid_points + s) / 2 + 1;
    println!("s = {}", s);

    Ok((p1, p2))
}
fn day17() -> io::Result<(u64, u64)> {
    let data = fs::read_to_string("data/day17.in").unwrap();
    let (mut p1, mut p2) = (0, 0);
    let dirs = [(1, 0), (0, 1), (-1, 0), (0, -1)];
    let grid = data
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    fn dijkstra(grid: &Vec<Vec<u32>>, minstep: isize, maxstep: isize) -> i64 {
        let mut location = (0, 0);
        let mut queue = BinaryHeap::from_iter([Reverse((0, 1, (0, 0), (0, 0)))]); // cost, step, location. direction
        let mut dist = HashMap::new(); // location, direction, step -> cost
        dist.insert(((0, 0), (0, 0), 1), 0);
        while let Some(Reverse(item)) = queue.pop() {
            let (cost, steps, node, dir) = item;
            let mut valid_dirs = match (dir) {
                (0, 1) => vec![(0, 1), (1, 0), (-1, 0)],
                (1, 0) => vec![(1, 0), (0, 1), (0, -1)],
                (0, -1) => vec![(0, -1), (1, 0), (-1, 0)],
                (-1, 0) => vec![(-1, 0), (0, 1), (0, -1)],
                _ => vec![(0, 1), (0, -1), (1, 0), (-1, 0)],
            };
            if steps == maxstep {
                // need to turn, remove forward dir
                valid_dirs.remove(0);
            }
            if steps < minstep {
                // need to keep going forward, remove others
                valid_dirs = vec![valid_dirs[0]];
            }
            for &(dx, dy) in &valid_dirs {
                let (x, y) = (node.0 + dx, node.1 + dy);
                if x < 0 || x >= grid.len() as i32 || y < 0 || y >= grid[0].len() as i32 {
                    continue;
                }
                let next_step = if dir == (dx, dy) { steps + 1 } else { 1 };
                let current = ((node.0, node.1), dir, steps);
                let next = ((x, y), (dx, dy), next_step);
                if !dist.contains_key(&next)
                    || dist[&next] > dist[&current] + grid[x as usize][y as usize]
                {
                    dist.insert(next, dist[&current] + grid[x as usize][y as usize]);
                    queue.push(Reverse((
                        dist[&current] + grid[x as usize][y as usize],
                        next_step,
                        (x, y),
                        (dx, dy),
                    )));
                }
            }
        }
        let mut min_cost = u32::MAX;
        for dir in [(1, 0), (0, 1), (-1, 0), (0, -1)] {
            for step in minstep..=maxstep {
                if dist.contains_key(&(
                    (grid.len() as i32 - 1, grid[0].len() as i32 - 1),
                    dir,
                    step,
                )) {
                    println!(
                        "{} - {:?}- {}",
                        step,
                        dir,
                        dist[&((grid.len() as i32 - 1, grid[0].len() as i32 - 1), dir, step)]
                    );
                    min_cost = min_cost
                        .min(dist[&((grid.len() as i32 - 1, grid[0].len() as i32 - 1), dir, step)]);
                }
            }
        }
        println!("----");
        return min_cost as i64;
    }
    p1 = dijkstra(&grid, 1, 3);
    p2 = dijkstra(&grid, 4, 10);
    Ok((p1 as u64, p2 as u64))
}
fn day16() -> io::Result<(u64, u64)> {
    let data = fs::read_to_string("data/day16.in").unwrap();
    let (mut p1, mut p2) = (0, 0);
    let grid = data
        .lines()
        .collect_vec()
        .iter()
        .map(|l| l.chars().collect_vec())
        .collect_vec();
    let mut seen = HashMap::new();
    fn solve_for_beam(
        grid: &Vec<Vec<char>>,
        seen: &mut HashMap<((usize, usize), (isize, isize)), (u32)>,
        silocation: (isize, isize),
        sdir: (isize, isize),
    ) {
        let mut stack = Vec::new();
        stack.push((silocation, sdir));
        while let Some((ilocation, dir)) = stack.pop() {
            if ilocation.0 < 0
                || ilocation.0 >= grid.len() as isize
                || ilocation.1 < 0
                || ilocation.1 >= grid[0].len() as isize
            {
                continue;
            }
            let location = (ilocation.0 as usize, ilocation.1 as usize);
            if seen.contains_key(&(location, dir)) {
                continue;
            }
            seen.insert((location, dir), 1);
            if grid[location.0][location.1] == '.' {
                stack.push(((ilocation.0 + dir.0, ilocation.1 + dir.1), dir));
            } else if grid[location.0][location.1] == '\\' {
                match dir {
                    (-1, 0) => stack.push(((ilocation.0, ilocation.1 - 1), (0, -1))),
                    (1, 0) => stack.push(((ilocation.0, ilocation.1 + 1), (0, 1))),
                    (0, 1) => stack.push(((ilocation.0 + 1, ilocation.1), (1, 0))),
                    (0, -1) => stack.push(((ilocation.0 - 1, ilocation.1), (-1, 0))),
                    _ => continue,
                }
            } else if grid[location.0][location.1] == '/' {
                match dir {
                    (-1, 0) => stack.push(((ilocation.0, ilocation.1 + 1), (0, 1))),
                    (1, 0) => stack.push(((ilocation.0, ilocation.1 - 1), (0, -1))),
                    (0, 1) => stack.push(((ilocation.0 - 1, ilocation.1), (-1, 0))),
                    (0, -1) => stack.push(((ilocation.0 + 1, ilocation.1), (1, 0))),
                    _ => continue,
                }
            } else if grid[location.0][location.1] == '-' {
                match dir {
                    (-1, 0) | (1, 0) => {
                        stack.push(((ilocation.0, ilocation.1 + 1), (0, 1)));
                        stack.push(((ilocation.0, ilocation.1 - 1), (0, -1)));
                    }
                    (0, 1) | (0, -1) => stack.push(((ilocation.0, ilocation.1 + dir.1), dir)),
                    _ => continue,
                }
            } else if grid[location.0][location.1] == '|' {
                match dir {
                    (-1, 0) | (1, 0) => stack.push(((ilocation.0 + dir.0, ilocation.1), dir)),
                    (0, 1) | (0, -1) => {
                        stack.push(((ilocation.0 + 1, ilocation.1), (1, 0)));
                        stack.push(((ilocation.0 - 1, ilocation.1), (-1, 0)));
                    }
                    _ => continue,
                }
            }
        }
    }
    solve_for_beam(&grid, &mut seen, (0, 0), (0, 1));
    p1 = seen
        .keys()
        .collect_vec()
        .iter()
        .map(|&&k| k.0)
        .unique()
        .collect_vec()
        .len() as u64;
    seen.clear();
    let mut max = 0;
    for i in 1..grid.len() - 1 {
        solve_for_beam(&grid, &mut seen, (0, i as isize), (0, 1));
        max = max.max(
            seen.keys()
                .collect_vec()
                .iter()
                .map(|&&k| k.0)
                .unique()
                .collect_vec()
                .len() as u64,
        );
        seen.clear();
    }
    for i in 1..grid.len() - 1 {
        solve_for_beam(
            &grid,
            &mut seen,
            (grid[0].len() as isize - 1, i as isize),
            (0, -1),
        );
        max = max.max(
            seen.keys()
                .collect_vec()
                .iter()
                .map(|&&k| k.0)
                .unique()
                .collect_vec()
                .len() as u64,
        );
        seen.clear();
    }
    for i in 1..grid[0].len() - 1 {
        solve_for_beam(&grid, &mut seen, (0, i as isize), (1, 0));
        max = max.max(
            seen.keys()
                .collect_vec()
                .iter()
                .map(|&&k| k.0)
                .unique()
                .collect_vec()
                .len() as u64,
        );
        seen.clear();
    }
    for i in 1..grid[0].len() - 1 {
        solve_for_beam(
            &grid,
            &mut seen,
            (grid.len() as isize - 1, i as isize),
            (-1, 0),
        );
        max = max.max(
            seen.keys()
                .collect_vec()
                .iter()
                .map(|&&k| k.0)
                .unique()
                .collect_vec()
                .len() as u64,
        );
        seen.clear();
    }

    for (aloc, adir) in [
        ((0, 0), (0, 1)),
        ((0, 0), (1, 0)),
        ((0, grid[0].len() - 1), (0, -1)),
        ((0, grid[0].len() - 1), (1, 0)),
        ((grid.len() - 1, 0), (0, 1)),
        ((grid.len() - 1, 0), (-1, 0)),
        ((grid.len() - 1, grid[0].len() - 1), (0, -1)),
        ((grid.len() - 1, grid[0].len() - 1), (-1, 0)),
    ] {
        solve_for_beam(&grid, &mut seen, (aloc.0 as isize, aloc.1 as isize), adir);
        max = max.max(
            seen.keys()
                .collect_vec()
                .iter()
                .map(|&&k| k.0)
                .unique()
                .collect_vec()
                .len() as u64,
        );
        seen.clear();
    }
    p2 = max;
    Ok((p1, p2))
}
fn day15() -> io::Result<(u64, u64)> {
    let data = fs::read_to_string("data/day15.in").unwrap();
    let (mut p1, mut p2) = (0, 0);
    let parts = data.lines().collect_vec()[0].split(",");
    fn hash(s: &str) -> u32 {
        s.chars().fold(0, |acc, c| ((acc + c as u32) * 17) % 256)
    }
    for part in parts.clone() {
        p1 += hash(part) as u64;
    }
    const N: usize = 256;
    let mut boxes: Vec<Vec<(String, u64)>> = vec![Vec::new(); 256];

    for part in parts.clone() {
        let label = part.split(['=', '-']).next().unwrap();
        let op = if part.contains("-") { '-' } else { '=' };
        let box_num = hash(label) as usize;

        if op == '=' {
            let value = part
                .split(['=', '-'])
                .nth(1)
                .unwrap()
                .parse::<u64>()
                .unwrap();
            let mut solved = false;
            for (i, boxy) in boxes[box_num].iter().enumerate() {
                if boxy.0 == label {
                    boxes[box_num][i] = (label.to_string(), value);
                    solved = true;
                    break;
                }
            }
            if !solved {
                boxes[box_num].push((label.to_string(), value));
            }
        } else if op == '-' {
            boxes[box_num].retain(|boxy| boxy.0 != label);
        }
    }
    for box_num in 0..N {
        if box_num >= boxes.len() || boxes[box_num].is_empty() {
            continue;
        }
        for (i, (label, value)) in boxes[box_num].iter().enumerate() {
            let power = (box_num as u64 + 1) * (i as u64 + 1) * value;
            println!(
                "box: {}, label: {}, value: {}, power: {}",
                box_num, label, value, power
            );
            p2 += power;
        }
    }
    Ok((p1, p2))
}

fn day14() -> io::Result<(u64, u64)> {
    let data = fs::read_to_string("data/day14.in").unwrap();
    let (mut p1, mut p2) = (0, 0);
    let mut mat = data
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    const N: u32 = 1_000_000_000;
    fn bring_north_south(mat: &mut Vec<Vec<char>>, dir: i32) {
        for i in 0..mat.len() {
            for j in 0..mat[i].len() {
                if mat[i][j] == 'O' {
                    let mut k = 1 * dir;
                    while (i as i32 + k) >= 0
                        && (i as i32 + k) < mat.len() as i32
                        && mat[(i as i32 + k) as usize][j] == '.'
                    {
                        k += 1 * dir;
                    }
                    if (i as i32 + k) == 0 {
                        k += 1;
                    }
                    if (i as i32 + k) == mat.len() as i32 {
                        k -= 1;
                    }
                    if k > 0 && (i as i32 + k) < mat.len() as i32 {
                        mat[(i as i32 + k) as usize][j] = 'O';
                        mat[i][j] = '.';
                    }
                }
            }
        }
    }

    fn bring_left_right(mat: &mut Vec<Vec<char>>, dir: i32) {
        for i in 0..mat.len() {
            for j in 0..mat[i].len() {
                if mat[i][j] == 'O' {
                    let mut k = 1 * dir;
                    while (i as i32 + k) >= 0
                        && (j as i32 + k) < mat[i].len() as i32
                        && mat[i][j + k as usize] == '.'
                    {
                        k += 1 * dir;
                    }
                    if (j as i32 + k) == 0 {
                        k += 1;
                    }
                    if (j as i32 + k) == mat[i].len() as i32 {
                        k -= 1;
                    }
                    if k > 0 && (j as i32 + k) < mat[i].len() as i32 {
                        mat[i][j + k as usize] = 'O';
                        mat[i][j] = '.';
                    }
                }
            }
        }
    }

    fn score(mat: &Vec<Vec<char>>) -> u64 {
        let mut score: u64 = 0;
        for i in 0..mat.len() {
            for j in 0..mat[i].len() {
                if mat[i][j] == 'O' {
                    score += (mat.len() - i) as u64;
                }
            }
        }
        score
    }
    // bring_north(&mut mat);
    // p1 = score(&mat);

    let mut history = Vec::new();
    for _ in 0..100 {
        bring_north_south(&mut mat, -1); // north
        bring_left_right(&mut mat, 1); // right
        bring_north_south(&mut mat, 1); // south
        bring_left_right(&mut mat, -1); // left
        history.push(mat.clone());
    }
    p2 = score(&mat);
    for i in 0..history.len() {
        for j in 0..history[i].len() {
            for k in 0..history[i][j].len() {
                print!("{}", history[i][j][k]);
            }
            println!();
        }
        println!("{} {}", i, score(&history[i]));
    }
    // for i in 0..mat.len() {
    //     for j in 0..mat[i].len() {
    //         print!("{}", mat[i][j]);
    //     }
    //     println!();
    // }
    Ok((p1, p2))
}
fn day13() -> io::Result<(u64, u64)> {
    let data = fs::read_to_string("data/day13.in").unwrap();
    let (mut p1, mut p2) = (0, 0);
    const N: usize = 50;
    let mut mat = [['.'; N]; N];
    let mut lines: usize = 0;
    let mut cols = 0;

    fn solve(mat: [[char; N]; N], lines: usize, cols: usize) -> u64 {
        let mut ans = 0;
        //find horizontal line
        for i in 0..cols as i32 {
            let mut is_reflection_line = true;
            let mut k = 1;
            let mut smuges = 0;
            while i - k + 1 >= 0 && i + k < cols as i32 && is_reflection_line {
                for j in 0..lines {
                    if mat[j][(i - k + 1) as usize] != mat[j][(i + k) as usize] {
                        smuges += 1;
                        if smuges > 1 {
                            is_reflection_line = false;
                            break;
                        }
                    }
                }
                k += 1;
            }
            if is_reflection_line && i != (cols - 1) as i32 && smuges == 1 {
                println!("horizontal line: {}", i + 1);
                ans += i + 1;
                break;
            }
        }
        for i in 0..lines as i32 {
            let mut is_reflection_line = true;
            let mut k = 1;
            let mut smuges = 0;
            while i - k + 1 >= 0 && i + k < lines as i32 && is_reflection_line {
                for j in 0..cols {
                    if mat[(i - k + 1) as usize][j] != mat[(i + k) as usize][j] {
                        smuges += 1;
                        if smuges > 1 {
                            is_reflection_line = false;
                            break;
                        }
                    }
                }
                k += 1;
            }
            if is_reflection_line && i != (lines - 1) as i32 && smuges == 1 {
                println!("vertical line: {}", i + 1);
                ans += (i + 1) * 100;
                break;
            }
        }
        ans as u64
    }
    for line in data.lines() {
        if line.is_empty() {
            p1 += solve(mat, lines, cols);
            (lines, cols) = (0, 0);
            continue;
        }
        for (j, c) in line.chars().enumerate() {
            mat[lines][j] = c;
        }
        cols = max(cols, line.chars().collect::<Vec<char>>().len());
        lines += 1;
    }
    p1 += solve(mat, lines, cols);
    Ok((p1, p2))
}
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
    }
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
    Ok((p1, p2))
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
        if start.0 == end.0 {
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
        } else if start.1 == end.1 {
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
    dbg!(day25()?);
    println!("Elapsed: {:?}us", now.elapsed().as_millis());
    Ok(())
}
