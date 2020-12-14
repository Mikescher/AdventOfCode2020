use crate::common::AdventOfCodeDay;

use std::collections::HashMap;

pub struct Day14 {
    input: Vec<String>,
}

impl Day14 {
    pub fn new() -> Self {
        let input_bytes = include_bytes!("../res/14_input.txt");
        let input_str = String::from_utf8_lossy(input_bytes);
        
        let lines = input_str
                        .lines()
                        .map(|p| String::from(p))
                        .collect::<Vec<String>>();

        Self {
            input: lines
        }
    }
}

impl AdventOfCodeDay for Day14 {

    fn task_1(&self) -> String {

        let mut mask_set:   u64 = 0;
        let mut mask_unset: u64 = 0;

        let mut mem: HashMap<u64, u64> = HashMap::new();

        for line in &self.input {

            if line.starts_with("mask = ") {

                let strmask = &line[7..];

                mask_set   = u64::from_str_radix(&strmask.replace("X", "0"),  2).unwrap();
                mask_unset = u64::from_str_radix(&strmask.replace("X", "1"),  2).unwrap();

                verboseln!("Mask: {} -> set:{} | unset:{}", strmask, mask_set, mask_unset);

            } else if line.starts_with("mem[") {

                let addr  = line.replace("mem[", "").replace("] =", "").split(' ').nth(0).unwrap().parse::<u64>().unwrap();
                let value = line.replace("mem[", "").replace("] =", "").split(' ').nth(1).unwrap().parse::<u64>().unwrap();

                let masked_val = (value | mask_set) & mask_unset;

                mem.insert(addr, masked_val);
                
                verboseln!("Set: {} := {} (orig: {})", addr, masked_val, value);
            }

        }

        return mem.iter().map(|(k,v)| v).sum::<u64>().to_string();
    }

    fn task_2(&self) -> String  {
        return "TODO".to_owned() //TODO
    }
}