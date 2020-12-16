use crate::common::AdventOfCodeDay;

use itertools::Itertools;

#[derive(Debug)]
pub struct Day16 {
    rules: Vec<Rule>,
    ticket: RawTicketData,
    nearby: Vec<RawTicketData>,
}

#[derive(Debug)]
struct Rule {
    name: String,
    ranges: Vec<(i32, i32)>,
}

impl Rule {
    fn matches(&self, dat: i32) -> bool {
        return self.ranges.iter().any(|(lower, upper)| dat >= *lower && dat <= *upper);
    }
}

#[derive(Debug)]
struct RawTicketData {
    fields: Vec<i32>,
}

impl Day16 {
    pub fn new() -> Self {
        let input_bytes = include_bytes!("../res/16_input.txt");
        let input_str = String::from_utf8_lossy(input_bytes);
        
        let lines = input_str
                        .lines()
                        .map(|p| String::from(p))
                        .collect::<Vec<String>>();

        let mut i = 0;

        let mut rules: Vec<Rule> = Vec::new();
        loop {
            i+=1;
            let line = lines[i].clone();
            if line == "" { i+=1; break; }

            let s1 = line.split(": ").collect::<Vec<&str>>();
            let name = s1[0].to_owned();

            let s2 = s1[1].split(" or ").collect::<Vec<&str>>();

            let s21 = s2[0].split('-').collect::<Vec<&str>>();
            let range1 = (s21[0].parse::<i32>().unwrap(), s21[1].parse::<i32>().unwrap());

            let s22 = s2[1].split('-').collect::<Vec<&str>>();
            let range2 = (s22[0].parse::<i32>().unwrap(), s22[1].parse::<i32>().unwrap());

            rules.push(Rule {
                name: name,
                ranges: vec![ range1, range2 ],
            })
        }

        i+=1;
        let myticket = RawTicketData {
            fields: lines[i].split(',').map(|p| p.parse::<i32>().unwrap()).collect(),
        };
        
        i+=1;
        i+=1;

        let mut nearbytickets: Vec<RawTicketData> = Vec::new();
        while i+1 < lines.len() {
            i+=1;

            nearbytickets.push(RawTicketData {
                fields: lines[i].split(',').map(|p| p.parse::<i32>().unwrap()).collect(),
            });
        }

        Self {
            rules: rules,
            ticket: myticket,
            nearby: nearbytickets,
        }
    }
}

impl AdventOfCodeDay for Day16 {

    fn task_1(&self) -> String {
        return self.nearby
                   .iter()
                   .flat_map(|p| p.fields.iter())
                   .filter(|f| self.rules.iter().all(|r| !r.matches(**f)))
                   .sum::<i32>()
                   .to_string();
    }

    fn task_2(&self) -> String  {
        return "TODO".to_owned() //TODO
    }
}