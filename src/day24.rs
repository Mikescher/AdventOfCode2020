use crate::common::AdventOfCodeDay;

use std::collections::HashMap;

#[derive(Debug, PartialEq, Clone)]
enum HexDir {
    EAST,
    SOUTHEAST,
    SOUTHWEST,
    WEST,
    NORTHWEST,
    NORTHEAST,
}

#[derive(Debug)]
pub struct Day24 {
    input: Vec<Vec<HexDir>>,
}

impl Day24 {
    pub fn new() -> Self {
        let input_bytes = include_bytes!("../res/24_input.txt");
        let input_str = String::from_utf8_lossy(input_bytes);
        
        let data = input_str
                        .lines()
                        .map(|p| String::from(p))
                        .map(|p| Self::parse_line(p))
                        .collect::<Vec<_>>();

        Self {
            input: data
        }
    }

    fn parse_line(line: String) -> Vec<HexDir> {
        let mut r = Vec::new();

        let mut skip = false;
        for u in 0..line.len() {
            if skip { skip = false; continue; }

            let chr = line.chars().nth(u+0).unwrap();
            let nxt = line.chars().nth(u+1).unwrap_or(' ');

            r.push(match (chr,nxt) {
                ('e', _)   => { skip=false; HexDir::EAST },
                ('s', 'e') => { skip=true;  HexDir::SOUTHEAST },
                ('s', 'w') => { skip=true;  HexDir::SOUTHWEST },
                ('w', _)   => { skip=false; HexDir::WEST },
                ('n', 'w') => { skip=true;  HexDir::NORTHWEST },
                ('n', 'e') => { skip=true;  HexDir::NORTHEAST },

                _ => panic!(),
            });
        }

        return r;
    }
}

#[derive(Debug, PartialEq, Clone, Hash, Eq)]
struct HexCoordOddR {
    q: i32,
    r: i32,
}

impl HexCoordOddR {
    pub fn zero() -> Self {
        return Self {
            q: 0,
            r: 0,
        }
    }

    pub fn move_by(&self, d: &HexDir) -> Self {
        return match d {
            HexDir::EAST      => Self{ q: self.q+1,                   r: self.r   },
            HexDir::SOUTHEAST => Self{ q: self.q+realmod(self.r,2),   r: self.r+1 },
            HexDir::SOUTHWEST => Self{ q: self.q+realmod(self.r,2)-1, r: self.r+1 },
            HexDir::WEST      => Self{ q: self.q-1,                   r: self.r   },
            HexDir::NORTHWEST => Self{ q: self.q+realmod(self.r,2)-1, r: self.r-1 },
            HexDir::NORTHEAST => Self{ q: self.q+realmod(self.r,2),   r: self.r-1 },
        }
    }
}

fn realmod(v: i32, m: i32) -> i32 {
    return ((v % m) + m) % m
}

impl AdventOfCodeDay for Day24 {

    fn task_1(&self) -> String {

        let mut grid = HashMap::<HexCoordOddR, bool>::new();

        for path in &self.input {
            
            let mut coord = HexCoordOddR::zero();
            for step in path { 
                let coord2 = coord.move_by(step); 
                verboseln!("  Move [{},{}] --[{:?}]--> [{},{}]", coord.q, coord.r, step, coord2.q, coord2.r);
                coord = coord2;
            }

            let state = !*grid.get(&coord).unwrap_or(&false);

            verboseln!("Set [{},{}] -> {}", coord.q, coord.r, state);

            grid.insert(coord.clone(), state);
        }

        return grid.iter().filter(|(_, v)| **v).count().to_string();
    }

    fn task_2(&self) -> String  {
        return "TODO".to_owned() //TODO
    }
}