use regex::{Regex, RegexSet};
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::vec;

#[derive(Debug)]
struct Subset {
    red: u32,
    green: u32,
    blue: u32,
}

impl Subset {
    fn new(s: &str) -> Subset {
        let re = Regex::new(r"(\d+)\s(red|green|blue)").unwrap();
        let mut rgb = [0, 0, 0];
        for (_, [n, color]) in re.captures_iter(s).map(|caps| caps.extract()) {
            let n = n.parse::<u32>().unwrap();
            match color {
                "red" => rgb[0] += n,
                "green" => rgb[1] += n,
                "blue" => rgb[2] += n,
                _ => (),
            }
        }

        Subset {red: rgb[0], green: rgb[1], blue: rgb[2]}
    }
}

fn main() {
    if let Ok(lines) = read_lines("./inputs/input2.txt") {
        for line in lines {
            if let Ok(s) = line {
                let elements: Vec<&str> = s.split(":").collect();
                let game_id: u32 = elements.first().unwrap().split(" ").collect::<Vec<&str>>().last().unwrap().parse::<u32>().unwrap();

                println!("{:?}", game_id);
                let subsets: Vec<&str> = elements.last().unwrap().split(";").collect();

                for s in subsets {
                    let subset = Subset::new(s);
                    println!("{:?}", subset);
                }
            }
        }
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}