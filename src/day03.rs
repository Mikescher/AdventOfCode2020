use crate::common::AdventOfCodeDay;

pub struct Day03 {
    width: i32,
    height: i32,
    input: Vec<Vec<bool>>,
}

impl Day03 {
    pub fn new() -> Self {
        let input_bytes = include_bytes!("../res/03_input.txt");
        let input_str = String::from_utf8_lossy(input_bytes);
        
        let lines = input_str.lines().map(String::from).collect::<Vec<String>>();

        Self {
            width: lines[0].len() as i32,
            height: lines.len() as i32,

            input: lines.iter().map(|l| l.chars().map(|c| c=='#').collect()).collect(),
        }
    }

    pub fn get(&self, x: i32, y: i32) -> bool {
        return self.input[y as usize][(x % self.width) as usize]
    }
}

impl AdventOfCodeDay for Day03 {

    fn task_1(&self) -> String {
        (0..self.height).filter(|y| self.get(3*y, *y)).count().to_string()
    }

    fn task_2(&self) -> String  {
        return "TODO".to_owned() //TODO
    }
}