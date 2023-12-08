use regex::{Regex, RegexSet};
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::vec;

fn main() {
    if let Ok(lines) = read_lines("./inputs/input1.txt") {
        let mut sum1: u32 = 0;
        let mut sum2: u32 = 0;

        for line in lines {
            if let Ok(s) = line {
                let number = merge_digits_1(&s);
                sum1 += number;

                let number = merge_digits_2(&s);
                sum2 += number;
            }
        }

        println!("{}", sum1);
        println!("{}", sum2);
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn merge_digits_1(line: &String) -> u32 {
    let digits: Vec<String> = line
        .chars()
        .filter(|c| c.is_numeric())
        .map(|c| c.to_string())
        .collect();
    let first = digits.first().unwrap().to_owned();
    let last = digits.last().unwrap();
    (first + last).parse::<u32>().unwrap()
}

fn merge_digits_2(line: &String) -> u32 {
    let digits_hm = HashMap::from([
        ("one", "1"),
        ("two", "2"),
        ("three", "3"),
        ("four", "4"),
        ("five", "5"),
        ("six", "6"),
        ("seven", "7"),
        ("eight", "8"),
        ("nine", "9"),
    ]);

    let mut keys : Vec<&str> = digits_hm.keys().copied().collect::<Vec<&str>>();
    keys.append(vec![r"\d"].as_mut());

    let set = RegexSet::new(
        keys
    ).unwrap();

    let regexes: Vec<_> = set
        .patterns()
        .iter()
        .map(|pat| Regex::new(pat).unwrap())
        .collect();
    // let re = Regex::new(r"(\d|one|two|three|four|five|six|seven|eight|nine)").unwrap();

    // let mut results = vec![];
    let mut matches: Vec<(usize, &str)> = set
        .matches(line)
        .into_iter()
        .map(|index| &regexes[index])
        .map(|re| re
            .find_iter(line)
            .map(|m| (m.start(), m.as_str()))
            .collect::<Vec<(usize, &str)>>()
            )
        .flatten()
        .collect();

    matches.sort_by_key(|k| k.0);
    let matches: Vec<&str> = matches.into_iter().map(|x| x.1).collect();

    let mut results : Vec<&str> = vec![];
    for m in matches {
        let digit = match m.parse::<u32>() {
            Ok(_x) => m,
            Err(_x) => digits_hm[m],
        };

        results.append(&mut vec![digit]);
    }

    // println!("{:?}", results);
    let first = results.first().unwrap().to_owned().to_string();
    let last = results.last().unwrap().to_owned();

    (first + last).parse::<u32>().unwrap()
}
