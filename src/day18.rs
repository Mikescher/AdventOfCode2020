use crate::common::AdventOfCodeDay;

pub struct Day18 {
    input: Vec<String>,
}

impl Day18 {
    pub fn new() -> Self {
        let input_bytes = include_bytes!("../res/18_input.txt");
        let input_str = String::from_utf8_lossy(input_bytes);
        
        let lines = input_str
                        .lines()
                        .map(|p| String::from(p))
                        .collect::<Vec<String>>();

        Self {
            input: lines
        }
    }

    fn full_eval(formula: String) -> i64 {
        return Self::eval(&formula.chars().filter(|p| *p != ' ').collect(), 0).0;
    }

    fn eval(formula: &Vec<char>, start_idx: usize) -> (i64, usize) {

        let mut curr: i64 = 0;

        let mut i = start_idx;
        loop {
            if i >= formula.len() || formula[i] == ')' {
                return (curr, i+1);
            }

            let mut op = '+';

            if i > start_idx {
                op = formula[i];
                i += 1;
            }

            let param: i64;
            if formula[i] == '(' {
                (param, i) = Self::eval(formula, i+1);
            } else {
                param = formula[i].to_string().parse::<i64>().unwrap();
                i += 1;
            }

            match op {
                '+' => { curr += param; }
                '*' => { curr *= param; }
                _   => panic!()
            }
        }
    }
}

impl AdventOfCodeDay for Day18 {

    fn task_1(&self) -> String {

        if is_verbose!() {
            for line in &self.input {
                verboseln!("{}   :=   {:?}", line, Day18::full_eval(line.to_owned()));
            }
        }

        return self.input.iter().map(|p| Day18::full_eval(p.to_owned())).sum::<i64>().to_string();

    }

    fn task_2(&self) -> String  {
        return "TODO".to_owned() //TODO
    }
}