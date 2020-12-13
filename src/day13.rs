use crate::common::AdventOfCodeDay;

pub struct Day13 {
    input_time: u32,
    input_bus: Vec<Option<u32>>,
}

impl Day13 {
    pub fn new() -> Self {
        let input_bytes = include_bytes!("../res/13_input.txt");
        let input_str = String::from_utf8_lossy(input_bytes);
        
        let t = input_str.lines().next().unwrap().parse::<u32>().unwrap();

        let b = input_str.lines().skip(1).next().unwrap().split(',').map(|p| p.parse::<u32>().ok()).collect::<Vec<Option<u32>>>();

        Self {
            input_time: t,
            input_bus: b,
        }
    }
}

impl AdventOfCodeDay for Day13 {

    fn task_1(&self) -> String {

        let mut b = self.input_bus
                .iter()
                .flat_map(|p| p.iter())
                .map(|p| *p)
                .collect::<Vec<u32>>();
        
        b.sort_by(|a,b| (self.input_time % b).cmp(&(self.input_time % a)));

        verboseln!("{}", self.input_time);
        verboseln!("{:?}", b);
        verboseln!("{:?}", b.iter().map(|p| p - self.input_time % p).collect::<Vec<u32>>());

        return (b[0] * (b[0] - self.input_time % b[0])).to_string();
    }

    fn task_2(&self) -> String  {
        return "TODO".to_owned() //TODO
    }
}