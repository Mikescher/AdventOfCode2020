
pub struct Day01 {
    input: Vec<i32>,
}

impl Day01 {
    pub fn new() -> Self {
        let input_bytes = include_bytes!("../res/01_input.txt");
        let input = String::from_utf8_lossy(input_bytes);
        
        let numbers = input.lines().map(|p| p.parse::<i32>().unwrap()).collect::<Vec<i32>>();

        Self {
            input: numbers
        }
    }

    pub fn task_1(&self) {
        for v1 in &self.input {
            for v2 in &self.input {
                if v1+v2 == 2020 {
                    println!("{}", v1*v2);
                    return;
                }
            }
        }
    }
}