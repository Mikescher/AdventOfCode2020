use crate::common::AdventOfCodeDay;

pub struct Day23 {
    input: Vec<u8>,
}

impl Day23 {
    pub fn new() -> Self {
        let input_bytes = include_bytes!("../res/23_input.txt");
        let input_str = String::from_utf8_lossy(input_bytes);
        
        let cups = input_str
                        .chars()
                        .map(|p| p.to_string().parse::<u8>().unwrap())
                        .collect::<Vec<u8>>();

        Self {
            input: cups
        }
    }
}

impl Day23 {
    fn do_move(cups: &mut Vec<u8>) {

        verboseln!("Cups: {:?}", cups);

        let n1 = cups[1];
        let n2 = cups[2];
        let n3 = cups[3];

        verboseln!("pick up: {}, {}, {}", n1,n2,n3);

        let mut dest = (cups[0] + 9-1-1) % 9 + 1;
        while !cups.iter().skip(4).any(|p| *p == dest) {
            dest = (dest + 9-1-1) % 9 + 1;
        }
        let dest_idx = cups.iter().position(|p| *p==dest).unwrap() + 1;

        verboseln!("destination: {}, (idx: {})", dest,dest_idx);

        cups.insert(dest_idx, n3);
        cups.insert(dest_idx, n2);
        cups.insert(dest_idx, n1);

        cups.remove(1);
        cups.remove(1);
        cups.remove(1);
        
        let f = cups.remove(0);
        cups.push(f);

        verboseln!();
    }
}

impl AdventOfCodeDay for Day23 {

    fn task_1(&self) -> String {

        let mut cups = self.input.clone();

        for _ in 0..100 {
            Self::do_move(&mut cups);
        }

        let c1idx = cups.iter().position(|p| *p==1).unwrap();
        
        let mut r = String::new();
        for idx in 1..9 {
            r = r + &cups[(idx+c1idx) % 9].to_string();
        }

        return r;
    }

    fn task_2(&self) -> String  {
        return "TODO".to_owned() //TODO
    }
}