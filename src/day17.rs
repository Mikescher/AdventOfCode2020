use crate::common::AdventOfCodeDay;

use std::collections::HashMap;
pub struct Day17 {
    width: i32,
    height: i32,
    input: Vec<Vec<bool>>,
}

impl Day17 {
    pub fn new() -> Self {
        let input_bytes = include_bytes!("../res/17_input.txt");
        let input_str = String::from_utf8_lossy(input_bytes);
        
        let lines = input_str.lines().map(String::from).collect::<Vec<String>>();

        Self {
            width: lines[0].len() as i32,
            height: lines.len() as i32,

            input: lines.iter().map(|l| l.chars().map(|c| c=='#').collect()).collect(),
        }
    }
}

struct PocketUniverse3D {
    state: HashMap<(i32, i32, i32), (bool, bool)>
}

impl PocketUniverse3D {
    pub fn new() -> Self {
        Self {
            state: HashMap::with_capacity(8192),
        }
    }

    fn step(&mut self) {
        let mut min_x = 0;
        let mut min_y = 0;
        let mut min_z = 0;
        let mut max_x = 0;
        let mut max_y = 0;
        let mut max_z = 0;

        for ((k_x, k_y, k_z), (v_old, v_curr)) in self.state.iter_mut() {
            *v_old = *v_curr;

            if *k_x < min_x { min_x = *k_x; }
            if *k_y < min_y { min_y = *k_y; }
            if *k_z < min_z { min_z = *k_z; }

            if *k_x > max_x { max_x = *k_x; }
            if *k_y > max_y { max_y = *k_y; }
            if *k_z > max_z { max_z = *k_z; }
        }

        for x in (min_x-1)..=(max_x+1) {
            for y in (min_y-1)..=(max_y+1) {
                for z in (min_z-1)..=(max_z+1) {
                    self.step_single(x, y, z);
                }
            }
        }
    }

    fn step_single(&mut self, x: i32, y: i32, z: i32) {

        let nbc = self.get_neighbours_old(x, y, z);

        if self.get_old(x,y,z) {

            if nbc == 2 || nbc == 3 {
                // stay active
            } else {
                self.set_new(x, y, z, false);
            }

        } else {

            if nbc == 3 {
                self.set_new(x, y, z, true);
            } else {
                // remain inactive
            }

        }
    }

    fn get_old(&self, x: i32, y: i32, z: i32) -> bool {
        if let Some((v_old, _)) = self.state.get(&(x,y,z)) {
            return *v_old;
        }
        return false;
    }

    fn get_new(&self, x: i32, y: i32, z: i32) -> bool {
        if let Some((_, v_new)) = self.state.get(&(x,y,z)) {
            return *v_new;
        }
        return false;
    }

    fn get_neighbours_old(&self, x: i32, y: i32, z: i32) -> i32 {
        let mut c = 0;

        for dx in -1..=1 {
            for dy in -1..=1 {
                for dz in -1..=1 {
                    if dx==0 && dy==0 && dz==0 {
                        continue;
                    }
                    if self.get_old(x+dx, y+dy, z+dz) {
                        c+=1;
                    }
                }
            }
        }

        return c;
    }

    fn set_new(&mut self, x: i32, y: i32, z: i32, v: bool) {
        if let Some((_, v_new)) = self.state.get_mut(&(x,y,z)) {
            *v_new = v;
        } else {
            self.state.insert((x,y,z), (false, v));
        }
    }

    fn print_verbose(&self) {
        let mut min_x = 0;
        let mut min_y = 0;
        let mut min_z = 0;
        let mut max_x = 0;
        let mut max_y = 0;
        let mut max_z = 0;

        for ((k_x, k_y, k_z), (_, _)) in self.state.iter() {
            if *k_x < min_x { min_x = *k_x; }
            if *k_y < min_y { min_y = *k_y; }
            if *k_z < min_z { min_z = *k_z; }

            if *k_x > max_x { max_x = *k_x; }
            if *k_y > max_y { max_y = *k_y; }
            if *k_z > max_z { max_z = *k_z; }
        }

        for z in min_z..=max_z {
            verboseln!("z={}", z);
            for y in min_y..=max_y {
                let mut str = String::with_capacity((max_x - min_x + 2) as usize);
                for x in min_x..=max_x {
                    if self.get_new(x, y, z) {
                        str.push_str("#");
                    } else {
                        str.push_str(".");
                    }
                }
                verboseln!("{}", str);
            }
            verboseln!();
        }

    }
}

struct PocketUniverse4D {
    state: HashMap<(i32, i32, i32, i32), (bool, bool)>
}

impl PocketUniverse4D {
    pub fn new() -> Self {
        Self {
            state: HashMap::with_capacity(8192),
        }
    }

    fn step(&mut self) {
        let mut min_x = 0;
        let mut min_y = 0;
        let mut min_z = 0;
        let mut min_w = 0;
        let mut max_x = 0;
        let mut max_y = 0;
        let mut max_z = 0;
        let mut max_w = 0;

        for ((k_x, k_y, k_z, k_w), (v_old, v_curr)) in self.state.iter_mut() {
            *v_old = *v_curr;

            if *k_x < min_x { min_x = *k_x; }
            if *k_y < min_y { min_y = *k_y; }
            if *k_z < min_z { min_z = *k_z; }
            if *k_w < min_w { min_w = *k_w; }

            if *k_x > max_x { max_x = *k_x; }
            if *k_y > max_y { max_y = *k_y; }
            if *k_z > max_z { max_z = *k_z; }
            if *k_w > max_w { max_w = *k_w; }
        }

        for x in (min_x-1)..=(max_x+1) {
            for y in (min_y-1)..=(max_y+1) {
                for z in (min_z-1)..=(max_z+1) {
                    for w in (min_w-1)..=(max_w+1) {
                        self.step_single(x, y, z, w);
                    }
                }
            }
        }
    }

    fn step_single(&mut self, x: i32, y: i32, z: i32, w: i32) {

        let nbc = self.get_neighbours_old(x, y, z, w);

        if self.get_old(x,y,z,w) {

            if nbc == 2 || nbc == 3 {
                // stay active
            } else {
                self.set_new(x, y, z, w, false);
            }

        } else {

            if nbc == 3 {
                self.set_new(x, y, z, w, true);
            } else {
                // remain inactive
            }

        }
    }

    fn get_old(&self, x: i32, y: i32, z: i32, w: i32) -> bool {
        if let Some((v_old, _)) = self.state.get(&(x,y,z,w)) {
            return *v_old;
        }
        return false;
    }

    fn get_new(&self, x: i32, y: i32, z: i32, w: i32) -> bool {
        if let Some((_, v_new)) = self.state.get(&(x,y,z,w)) {
            return *v_new;
        }
        return false;
    }

    fn get_neighbours_old(&self, x: i32, y: i32, z: i32, w: i32) -> i32 {
        let mut c = 0;

        for dx in -1..=1 {
            for dy in -1..=1 {
                for dz in -1..=1 {
                    for dw in -1..=1 {
                        if dx==0 && dy==0 && dz==0 && dw==0 {
                            continue;
                        }
                        if self.get_old(x+dx, y+dy, z+dz, w+dw) {
                            c+=1;
                        }
                    }
                }
            }
        }

        return c;
    }

    fn set_new(&mut self, x: i32, y: i32, z: i32, w: i32, v: bool) {
        if let Some((_, v_new)) = self.state.get_mut(&(x,y,z,w)) {
            *v_new = v;
        } else {
            self.state.insert((x,y,z,w), (false, v));
        }
    }
}

impl AdventOfCodeDay for Day17 {

    fn task_1(&self) -> String {

        let mut pu = PocketUniverse3D::new();

        for x in 0..self.width {
            for y in 0..self.height {
                pu.state.insert((x,y,0), (false, self.input[y as usize][x as usize]));
            }
        }

        if is_verbose!() {
            verboseln!("After 0 cycles:");
            verboseln!();
            pu.print_verbose();
        }

        for i in 0..6 {
            pu.step();

            if is_verbose!() {
                verboseln!("After {} cycles:", i+1);
                verboseln!();
                pu.print_verbose();
            }
        }

        return pu.state.iter().filter(|(_,v)| v.1).count().to_string();
    }

    fn task_2(&self) -> String  {
        let mut pu = PocketUniverse4D::new();

        for x in 0..self.width {
            for y in 0..self.height {
                pu.state.insert((x,y,0,0), (false, self.input[y as usize][x as usize]));
            }
        }

        for i in 0..6 {
            pu.step();
        }

        return pu.state.iter().filter(|(_,v)| v.1).count().to_string();
    }
}