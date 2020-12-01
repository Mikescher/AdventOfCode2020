mod common;

mod day01;
mod day02;

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