use crate::common::AdventOfCodeDay;

use regex::Regex;

#[derive(Debug)]
struct Data {
    key: String,
    value: String,
}

#[derive(Debug)]
struct Passport {
    data: Vec<Data>,
}

pub struct Day04 {
    input: Vec<Passport>,
}

fn parse_passport(p: String) -> Passport {
    let rex_values = Regex::new(r"\s+").unwrap();

    let data = rex_values
        .split(&p)
        .map(|q| q.split(':').map(String::from).collect::<Vec<String>>())
        .filter(|q| q.len() == 2)
        .map(|q| Data{ key: q[0].to_owned(), value: q[1].to_owned() })
        .collect::<Vec<Data>>();

    Passport {
        data: data
    }
}

impl Day04 {
    pub fn new() -> Self {
        let input_bytes = include_bytes!("../res/04_input.txt");
        let input_str = String::from_utf8_lossy(input_bytes);
        
        let rex_lines = Regex::new(r"(\r?\n){2}").unwrap();

        let lines = rex_lines.split(&input_str.to_owned())
                        .map(String::from)
                        .map(parse_passport)
                        .collect::<Vec<Passport>>();

        Self {
            input: lines
        }
    }
}

impl Passport {
    pub fn is_valid(&self) -> bool {
        if self.get("byr") == None { return false }
        if self.get("iyr") == None { return false }
        if self.get("eyr") == None { return false }
        if self.get("hgt") == None { return false }
        if self.get("hcl") == None { return false }
        if self.get("ecl") == None { return false }
        if self.get("pid") == None { return false }
        //if self.get("cid") == None { return false }

        return true
    }

    pub fn get(&self, key: &str) -> Option<String> {
        for dat in &self.data {
            if dat.key == key {
                return Some(String::from(&dat.value))
            }
        }
        return None
    }
}

impl AdventOfCodeDay for Day04 {
    fn task_1(&self) -> String {
        //for r in &self.input {
        //    println!("{:?}", r);
        //    println!();
        //}
        return self.input.iter().filter(|p| p.is_valid()).count().to_string() //TODO
    }

    fn task_2(&self) -> String  {
        return "TODO".to_owned() //TODO
    }
}