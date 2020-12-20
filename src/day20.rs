use crate::common::AdventOfCodeDay;

use regex::Regex;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;
use std::convert::TryInto;


#[derive(Debug, EnumIter, Clone, Copy)]
enum Compass { North, East, South, West }

#[derive(Clone, Copy)]
struct Tile {
    id: u32,
    bitmap: [[bool;10];10],
}

pub struct Day20 {
    input: [[Tile;12];12],
}

impl Day20 {
    pub fn new() -> Self {
        let input_bytes = include_bytes!("../res/20_input.txt");
        let input_str = String::from_utf8_lossy(input_bytes);
        
        let rex = Regex::new(r"Tile (?P<id>[0-9]+):\n(?P<bmp>([.#]{10}\n){10})").unwrap();

        let mut tiles = [[Tile{ id: 0, bitmap: [[false;10];10] };12];12];

        let mut i = 0;
        for cap in rex.captures_iter(&input_str)
        {
            tiles[i/12][i%12].id = cap.name("id").unwrap().as_str().parse::<u32>().unwrap();
            
            let raw = cap.name("bmp").unwrap().as_str().lines().map(|l| l.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();

            for y in 0..10 {
                for x in 0..10 {
                    tiles[i/12][i%12].bitmap[y][x] = raw[y][x]=='#';
                }
            }

            i+=1;
        }

        Self {
            input: tiles
        }
    }
}

impl Day20 {

    fn format_ids(&self) -> String {
        let mut r = String::new();
        for y in 0..12 {
            for x in 0..12 {
                let pad = format!(" {}", self.input[y][x].id);
                r.push_str(&pad);
            }
            r.push('\n');
        }
        return r;
    }


    fn format_bitmaps(&self) -> String {
        let mut r = String::with_capacity(12*12*12*12 + 1000);

        for y in 0..(12 * 11) {
            for x in 0..(12 * 11) {
            
                let gx = x / 11;
                let gy = y / 11;

                let ix = x % 11;
                let iy = y % 11;

                if ix==10 || iy == 10 { r.push(' '); continue; }

                r.push(match self.input[gy][gx].bitmap[iy][ix] {
                    true =>  '#',
                    false => '.',
                });
            }
            r.push('\n');
        }
        
        return r;
    }
}

impl Tile {
    fn format_bitmap(&self) -> String {
        let mut r = String::with_capacity(10*11);

        for y in 0..10 {
            for x in 0..10 {
                r.push(match self.bitmap[y][x] {
                    true =>  '#',
                    false => '.',
                });
            }
            r.push('\n');
        }
        return r;
    }

    fn get_side(&self, d: Compass) -> [bool;10] {
        let mut r = [false;10];

        match d {
            Compass::North => 
            {
                for i in 0..10 { r[i] = self.bitmap[0][i]; }
                return r;
            },
            Compass::East  =>
            {
                for i in 0..10 { r[i] = self.bitmap[i][0]; }
                return r;
            },
            Compass::South =>
            {
                for i in 0..10 { r[i] = self.bitmap[9][i]; }
                return r;
            },
            Compass::West  =>
            {
                for i in 0..10 { r[i] = self.bitmap[i][9]; }
                return r;
            },
        }
    }

    fn count_matching_sides(&self, tiles: &Vec<Tile>) -> u32 {

        return Compass::iter().filter(|d|
        {

            let side = self.get_side(*d);

            let mut c = 0;
            for tile in tiles.iter().filter(|t| t.id != self.id) {
                for d2 in Compass::iter() {
                    if Tile::sides_match_any(&side, &tile.get_side(d2)) {
                        c+=1;
                        break;
                    }
                }
            }
            return c > 0;

        }).count().try_into().unwrap();
    }

    fn sides_match_any(a: &[bool;10], b: &[bool;10]) -> bool
    {
        return Tile::sides_match_direct(a, b) || Tile::sides_match_reversed(a, b)
    }

    fn sides_match_direct(a: &[bool;10], b: &[bool;10]) -> bool
    {
        for i in 0..10 { if a[i] != b[i] { return false; } }
        return true;
    }

    fn sides_match_reversed(a: &[bool;10], b: &[bool;10]) -> bool
    {
        for i in 0..10 { if a[i] != b[9-i] { return false; } }
        return true;
    }
}

impl AdventOfCodeDay for Day20 {

    fn task_1(&self) -> String {

        verboseln!("{}", self.format_ids());
        verboseln!("{}", self.format_bitmaps());

        let tiles = self.input.iter().flat_map(|p| p.iter()).map(|p| *p).collect::<Vec<Tile>>();

        if is_verbose!() {
            for t in tiles.iter().filter(|t| t.count_matching_sides(&tiles) == 2) {
                verboseln!("{}", t.format_bitmap());
            }
        }

        return tiles.iter().filter(|t| t.count_matching_sides(&tiles) == 2).map(|p| p.id as u128).product::<u128>().to_string();
    }

    fn task_2(&self) -> String  {
        return "TODO".to_owned() //TODO
    }
}