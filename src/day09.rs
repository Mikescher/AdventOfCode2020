use crate::common::AdventOfCodeDay;

use std::collections::HashSet;

pub struct Day09 {
    input: Vec<u64>,
}

impl Day09 {
    pub fn new() -> Self {
        let input_bytes = include_bytes!("../res/09_input.txt");
        let input_str = String::from_utf8_lossy(input_bytes);
        
        let data = input_str
                       .lines()
                        .map(|p| p.parse::<u64>().unwrap())
                        .collect::<Vec<u64>>();

        Self {
            input: data
        }
    }

    pub fn all_combinations(data: Vec<&u64>) -> HashSet<u64> {
        let mut hs: HashSet<u64> = HashSet::new();

        for i1 in 0..24 {
            for i2 in (i1+1)..25 {
                hs.insert(data[i1] + data[i2]);
            }
        }

        return hs;
    }

    fn find_invalid(&self) -> u64 {
        for i in 25..self.input.len() {
            let comb = Day09::all_combinations(self.input.iter().skip(i-25).take(25).collect());
            if !comb.contains(&self.input[i]) {
                return self.input[i];
            }
            
        }

        panic!();
    }
}

impl AdventOfCodeDay for Day09 {

    fn task_1(&self) -> String {
        return self.find_invalid().to_string();
    }

    fn task_2(&self) -> String  {
        return "TODO".to_owned() //TODO
    }
}