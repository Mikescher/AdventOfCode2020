use crate::common::AdventOfCodeDay;

use itertools::Itertools;

pub struct Day10 {
    input: Vec<u32>,
}

impl Day10 {
    pub fn new() -> Self {
        let input_bytes = include_bytes!("../res/10_input.txt");
        let input_str = String::from_utf8_lossy(input_bytes);
        
        let data = input_str
                        .lines()
                        .map(|p| p.parse::<u32>().unwrap())
                        .collect::<Vec<u32>>();

        Self {
            input: data
        }
    }
}

impl AdventOfCodeDay for Day10 {

    fn task_1(&self) -> String {

        let (min, max) = self.input.iter().minmax().into_option().unwrap();

        verboseln!("min := {}", min);
        verboseln!("max := {}", max);

        let diff = self.input
                       .iter()
                       .chain([0, *max + 3].iter())
                       .sorted()
                       .collect::<Vec<&u32>>()
                       .windows(2)
                       .map(|p| p[1] - p[0])
                       .collect::<Vec<u32>>();

        let c1 = diff.iter().filter(|v| **v == 1).count();
        let c3 = diff.iter().filter(|v| **v == 3).count();

        verboseln!("{} * {} = {}", c1, c3, c1*c3);

        return (c1 * c3).to_string() //TODO
    }

    fn task_2(&self) -> String  {
        return "TODO".to_owned() //TODO
    }
}