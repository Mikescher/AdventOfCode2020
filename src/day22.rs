use crate::common::AdventOfCodeDay;

use regex::Regex;

#[derive(Debug)]
pub struct Day22 {
    cards_p1: Vec<u32>,
    cards_p2: Vec<u32>,
}

impl Day22 {
    pub fn new() -> Self {
        let input_bytes = include_bytes!("../res/22_input.txt");
        let input_str = String::from_utf8_lossy(input_bytes);

        let rex_lines = Regex::new(r"(\r?\n){2}").unwrap();
        let split = rex_lines.split(&input_str).collect::<Vec<&str>>();

        Self {
            cards_p1: split[0].lines().skip(1).map(|p| p.parse::<u32>().unwrap()).collect(),
            cards_p2: split[1].lines().skip(1).map(|p| p.parse::<u32>().unwrap()).collect(),
        }
    }
}

struct Game {
    cards_p1: Vec<u32>,
    cards_p2: Vec<u32>,
}

impl Game {
    fn play_one(&mut self) {
        let c1 = self.cards_p1.remove(0);
        let c2 = self.cards_p2.remove(0);

        if c1 > c2 {
            self.cards_p1.push(c1);
            self.cards_p1.push(c2);
        } else {
            self.cards_p2.push(c2);
            self.cards_p2.push(c1);
        }
    }
}

impl AdventOfCodeDay for Day22 {

    fn task_1(&self) -> String {

        let mut game = Game { cards_p1: self.cards_p1.clone(), cards_p2: self.cards_p2.clone(), };

        verboseln!("Round[0]:");
        verboseln!("    P1: {:?}", game.cards_p1);
        verboseln!("    P2: {:?}", game.cards_p2);
        verboseln!();

        for i in 1.. {

            game.play_one();

            verboseln!("Round[{}]:", i);
            verboseln!("    P1: {:?}", game.cards_p1);
            verboseln!("    P2: {:?}", game.cards_p2);
            verboseln!();

            if game.cards_p1.is_empty() || game.cards_p2.is_empty() { break; }
        }

        return game.cards_p1.iter()
                            .chain(game.cards_p2.iter())
                            .collect::<Vec<_>>()
                            .iter()
                            .rev()
                            .enumerate()
                            .map(|(i,v)| ((i+1) as u128, **v as u128))
                            .map(|(i,v)| i*v)
                            .fold(0, |a,b| a + b)
                            .to_string();
    }

    fn task_2(&self) -> String  {
        return "TODO".to_owned() //TODO
    }
}