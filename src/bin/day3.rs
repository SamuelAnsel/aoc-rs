use regex::Regex;
use aoc_rs::read_lines;

struct Schematic {
    elems: Vec<String>,
    n_row: usize,
    n_col: usize,
}

impl Schematic {
    fn new(filepath: &str) -> Schematic {
        let mut elems: Vec<String> = Vec::<String>::new();    

        if let Ok(lines) = read_lines(filepath) {
            for line in lines {
                if let Ok(s) = line {
                    elems.push(s);
                }
            }   
        }

        let n_col: usize = elems.first().unwrap().len();
        let n_row: usize = elems.len();

        Schematic {elems: elems, n_row: n_row, n_col: n_col}
    }

    fn get_part_numbers(&self) -> Vec<u32> {
        let mut part_numbers = Vec::<u32>::new();
        let re = Regex::new(r"\d+").unwrap();

        for (row, line) in self.elems.iter().enumerate() {
            for m in re.find_iter(line) {
                if self.is_span_adjacent(row, m.start(), m.end()) {
                    part_numbers.push(m.as_str().parse::<u32>().unwrap())
                }
            }
        }

        part_numbers
    }

    fn get_gear_ratios(&self) -> Vec<u32> {
        let mut gear_ratios = Vec::<u32>::new();
        let re = Regex::new(r"\*").unwrap();

        for (row, line) in self.elems.iter().enumerate() {
            for m in re.find_iter(line) {
                if self.is_span_adjacent(row, m.start(), m.end()) {
                    part_numbers.push(m.as_str().parse::<u32>().unwrap())
                }
            }
        }

        gear_ratios
    }

    fn is_adjacent(&self, row: usize, col: usize) -> bool {
        if col > 0 && row > 0 && self.is_symbol(row - 1 , col - 1) {return true;}
        if col > 0 && self.is_symbol(row, col - 1) {return true;}
        if col > 0 && row < self.n_row - 1 && self.is_symbol(row + 1 , col - 1) {return true;}
        if row > 0 && self.is_symbol(row - 1 , col) {return true;}
        if row < self.n_row - 1 && self.is_symbol(row + 1 , col) {return true;}
        if row > 0 && col < self.n_col - 1 && self.is_symbol(row - 1 , col + 1) {return true;}
        if col < self.n_col - 1 && self.is_symbol(row, col + 1) {return true;}
        if row < self.n_row - 1 && col < self.n_col - 1 && self.is_symbol(row + 1 , col + 1) {return true;}

        false
    }

    fn is_symbol(&self, row: usize, col: usize) -> bool {
        if self.elems[row].chars().nth(col).unwrap() == '.' {
            return false;
        }

        if self.elems[row].chars().nth(col).unwrap().is_numeric() {
            return false;
        }

        true
    }

    fn is_span_adjacent(&self, row: usize, start: usize, end: usize) -> bool {
        for i in start..end {
            if self.is_adjacent(row, i) {
                return true;
            }
        }

        false
    }
}

fn main() {
    let schem = Schematic::new("./inputs/input3.txt");
    let part_numbers = schem.get_part_numbers();
    let sum: u32 = part_numbers.iter().sum();
    println!("{:?}", sum);
}