mod common;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day20;
mod day21;
mod day22;
mod day23;
mod day24;
mod day25;

use std::env;
use std::io;
use std::io::prelude::*;

fn main() {
    let args: Vec<_> = env::args().collect();

    let day: i32;
    let tsk: i32;

    if args.len() <= 1 {
        day = input_int("Day");
        tsk = input_int("Task");
    } else if args.len() == 2 {
        day = args[1].parse::<i32>().expect("could not parse param_1");
        tsk = -1;
    } else if args.len() >= 3 {
        day = args[1].parse::<i32>().expect("could not parse param_1");
        tsk = args[2].parse::<i32>().expect("could not parse param_2");
    } else {
        panic!()
    }

    let dat = get_day(day);

    if tsk == -1 || tsk == 1 {
        println!("Day {} - Task 1: {}", day, dat.task_1());
    }
    if tsk == -1 || tsk == 2 {
        println!("Day {} - Task 2: {}", day, dat.task_2());
    }

}

fn get_day(day: i32) -> Box<dyn common::AdventOfCodeDay> {
    match day {
         1 => Box::new(day01::Day01::new()) as Box<dyn common::AdventOfCodeDay>,
         2 => Box::new(day02::Day02::new()) as Box<dyn common::AdventOfCodeDay>,
         3 => Box::new(day03::Day03::new()) as Box<dyn common::AdventOfCodeDay>,
         4 => Box::new(day04::Day04::new()) as Box<dyn common::AdventOfCodeDay>,
         5 => Box::new(day05::Day05::new()) as Box<dyn common::AdventOfCodeDay>,
         6 => Box::new(day06::Day06::new()) as Box<dyn common::AdventOfCodeDay>,
         7 => Box::new(day07::Day07::new()) as Box<dyn common::AdventOfCodeDay>,
         8 => Box::new(day08::Day08::new()) as Box<dyn common::AdventOfCodeDay>,
         9 => Box::new(day09::Day09::new()) as Box<dyn common::AdventOfCodeDay>,
        10 => Box::new(day10::Day10::new()) as Box<dyn common::AdventOfCodeDay>,
        11 => Box::new(day11::Day11::new()) as Box<dyn common::AdventOfCodeDay>,
        12 => Box::new(day12::Day12::new()) as Box<dyn common::AdventOfCodeDay>,
        13 => Box::new(day13::Day13::new()) as Box<dyn common::AdventOfCodeDay>,
        14 => Box::new(day14::Day14::new()) as Box<dyn common::AdventOfCodeDay>,
        15 => Box::new(day15::Day15::new()) as Box<dyn common::AdventOfCodeDay>,
        16 => Box::new(day16::Day16::new()) as Box<dyn common::AdventOfCodeDay>,
        17 => Box::new(day17::Day17::new()) as Box<dyn common::AdventOfCodeDay>,
        18 => Box::new(day18::Day18::new()) as Box<dyn common::AdventOfCodeDay>,
        19 => Box::new(day19::Day19::new()) as Box<dyn common::AdventOfCodeDay>,
        20 => Box::new(day20::Day20::new()) as Box<dyn common::AdventOfCodeDay>,
        21 => Box::new(day21::Day21::new()) as Box<dyn common::AdventOfCodeDay>,
        22 => Box::new(day22::Day22::new()) as Box<dyn common::AdventOfCodeDay>,
        23 => Box::new(day23::Day23::new()) as Box<dyn common::AdventOfCodeDay>,
        24 => Box::new(day24::Day24::new()) as Box<dyn common::AdventOfCodeDay>,
        25 => Box::new(day25::Day25::new()) as Box<dyn common::AdventOfCodeDay>,

        _ => panic!("day not found"),
    }
}

fn input_int(prompt: &str) -> i32 {
    loop {
        print!("{}: ", prompt);
        io::stdout().flush().unwrap();

        let mut line = String::new();
        let r = io::stdin().read_line(&mut line);
        if r.is_ok() {
            let num = line.trim().parse::<i32>();
            if num.is_ok() {
                return num.unwrap();
            }
        }
    }
}