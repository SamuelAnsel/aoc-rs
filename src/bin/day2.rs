use regex::{Regex, RegexSet};
use std::collections::HashMap;
use std::vec;
use aoc_rs::read_lines;

#[derive(Debug, Clone)]
struct Subset {
    red: u32,
    green: u32,
    blue: u32,
}

impl Subset {
    fn new(s: &str) -> Subset {
        let re: Regex = Regex::new(r"(\d+)\s(red|green|blue)").unwrap();
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
        let mut id_sum: u32 = 0;
        let mut power_sum: u32 = 0;

        for line in lines {
            if let Ok(s) = line {
                let elements: Vec<&str> = s.split(":").collect();
                let game_id: u32 = elements.first().unwrap().split(" ").collect::<Vec<&str>>().last().unwrap().parse::<u32>().unwrap();

                let subsets: Vec<Subset> = elements.last().unwrap().split(";").map(|s| Subset::new(s)).collect();
                let mut possible: bool = true;
                let mut min_subset: Subset = subsets.first().unwrap().clone();

                for subset in subsets {
                    if subset.red > 12 || subset.green > 13 || subset.blue > 14 {
                        possible = false;
                    }

                    if subset.red > min_subset.red {min_subset.red = subset.red;}
                    if subset.green > min_subset.green {min_subset.green = subset.green;}
                    if subset.blue > min_subset.blue {min_subset.blue = subset.blue;}
                 }

                if possible {
                    id_sum += game_id;
                }

                power_sum += min_subset.red * min_subset.green * min_subset.blue;
            }
        }

        println!("{:?}", id_sum);
        println!("{:?}", power_sum);
    }
}
