use std::fs;

fn day1() {
    let data = fs::read_to_string("data/day1.in").unwrap();
    let mut lines =data.lines();
    let mut sum = 0;
    let digits = vec!["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
    let parse_line = |line: &str| -> String {
        let mut result = line.to_string();
        let parsing = |result: String, rev: bool| -> String {
            let mut aux = result;
            loop {
                let mut earliest: Option<(usize, usize)> = None;
                for (i, &digit) in digits.iter().enumerate() {
                    let mut tofind = digit;
                    let second_aux = digit.chars().rev().collect::<String>();
                    if rev {
                        tofind = second_aux.as_str();
                    }
                    if let Some(pos) = aux.find(tofind) {
                        if earliest.is_none() || pos < earliest.unwrap().0 {
                            earliest = Some((pos, i));
                        }
                    }
                }

                match earliest {
                    Some((pos, index)) => {
                        aux.replace_range(pos..pos + digits[index].len(), (index + 1).to_string().as_str());
                    }
                    None => break
                }
            }
            aux
        };
        result = parsing(line.to_string(), false);
        result += parsing(line.to_string().chars().rev().collect::<String>(), true).chars().rev().collect::<String>().as_str();
        result
    };
    lines.for_each(|line| sum += parse_line(line).chars().find(|&c| c.is_ascii_digit()).unwrap().to_digit(10).unwrap() * 10 + parse_line(line).chars().rev().find(|&c| c.is_ascii_digit()).unwrap().to_digit(10).unwrap());
    println!("{}", sum);
}
fn main() {
   day1();
}
