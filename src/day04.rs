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
    pub fn is_all_present(&self) -> bool {
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

    pub fn is_all_valid(&self) -> bool {
        if self.get_and_validate("byr", r"^[12][90][0-9][0-9]$") == None { return false }
        if self.get_and_validate("iyr", r"^20[12][0-9]$") == None { return false }
        if self.get_and_validate("eyr", r"^20[23][0-9]$") == None { return false }
        if self.get_and_validate("hgt", r"^[0-9]+(in|cm)$") == None { return false }
        if self.get_and_validate("hcl", r"^#[0-9a-f]{6}$") == None { return false }
        if self.get_and_validate("ecl", r"^(amb|blu|brn|gry|grn|hzl|oth)$") == None { return false }
        if self.get_and_validate("pid", r"^[0-9]{9}$") == None { return false }
        
        let byr = self.get("byr").unwrap().parse::<i32>().unwrap();
        let iyr = self.get("iyr").unwrap().parse::<i32>().unwrap();
        let eyr = self.get("eyr").unwrap().parse::<i32>().unwrap();
        let hgt = self.get("hgt").unwrap();

        if byr < 1920 { return false }
        if byr > 2002 { return false }

        if iyr < 2010 { return false }
        if iyr > 2020 { return false }

        if eyr < 2020 { return false }
        if eyr > 2030 { return false }

        if hgt.ends_with("cm") {

            let v = hgt.replace("cm", "").parse::<i32>().unwrap();
            if v < 150 { return false }
            if v > 193 { return false }

        } else { //in

            let v = hgt.replace("in", "").parse::<i32>().unwrap();
            if v < 59 { return false }
            if v > 76 { return false }

        }

        return true
    }

    pub fn get_and_validate(&self, key: &str, rex: &str) -> Option<String> {
        if let Some(val) = self.get(key) {
            let rex = Regex::new(rex).unwrap();
            if rex.is_match(&val) {
                return Some(val)
            }
        }
        return None
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
        return self.input.iter().filter(|p| p.is_all_present()).count().to_string()
    }

    fn task_2(&self) -> String  {
        return self.input.iter().filter(|p| p.is_all_valid()).count().to_string()
    }
}