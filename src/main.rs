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
    let sysargs: Vec<_> = env::args().collect();

    let is_help = sysargs.iter().skip(1).any(|p| p == "--help") || 
                  sysargs.iter().skip(1).any(|p| p == "-h");

    let is_bench = sysargs.iter().skip(1).any(|p| p == "--benchmark") || 
                   sysargs.iter().skip(1).any(|p| p == "--bench") || 
                   sysargs.iter().skip(1).any(|p| p == "-b");

    let args : Vec<String> = sysargs.iter().skip(1).filter(|p| ! p.starts_with('-')).map(String::from).collect();

    if args.len() > 0 && (args[0] == "help" || is_help) {
        print_help();
        return;
    }

    if args.len() == 0 {
        print_help();
        
        println!();

        let day = input_int("Day");
        let tsk = input_int("Task");
        run_normal(day, tsk, false);
        return;
    }
    
    if args.len() == 1 && args[0] == "all" {
        for i in 1..26 { run_normal(i, -1, is_bench); }
        return;
    } 
    
    if args.len() == 1 && args[0] == "table" {
        print_table();
        return;
    } 
    
    if args.len() == 1 {
        let day = args[0].parse::<i32>().expect("could not parse param_1");
        run_normal(day, -1, is_bench);
        return;
    } 
    
    if args.len() == 2 {
        let day = args[0].parse::<i32>().expect("could not parse param_1");
        let tsk = args[1].parse::<i32>().expect("could not parse param_2");
        run_normal(day, tsk, is_bench);
        return;
    }
    
    println!();
    println!("Invalid arguments supplied, use --help for commandline documentation");
    println!();
}

fn print_help() {
    println!("Advent of code 2020 /by Mikescher.");
    println!();
    println!("Usage: ");
    println!("  advent_of_code_2020 help");
    println!("  advent_of_code_2020 <day>");
    println!("  advent_of_code_2020 <day> <task>");
    println!("  advent_of_code_2020 all");
    println!("  advent_of_code_2020 table");
    println!();
    println!("Options:");
    println!("  -h --help");
    println!("  -b --benchmark");
}

fn run_normal(day: i32, tsk: i32, benchmark: bool) {
    if tsk == -1 || tsk == 1 { 
        if benchmark { run_benchmark(day, 1); } else { run_single(day, 1); }
    }

    if (tsk == -1 && day != 25) || tsk == 2 { 
        if benchmark { run_benchmark(day, 2); } else { run_single(day, 2); }
    }
}

fn run_single(day: i32, tsk: i32) {
    if day == 25 && tsk == 2 { panic!("No day 25 - Task 2"); }

    let dat = get_day(day);

    if tsk == 1 {
        println!("Day {:0>2} - Task 1: {}", day, dat.task_1());
    } else if tsk == 2 {
        println!("Day {:0>2} - Task 2: {}", day, dat.task_2());
    } else {
        panic!();
    }
}

fn run_benchmark(day: i32, tsk: i32) {
    use std::time::*;

    println!();
    println!("> Start benchmark for day {:0>2} task {}", day, tsk);
    
    let swatch = Instant::now();
    let mut count = 0;

    let mut times : Vec<Duration> = Vec::with_capacity(2048);

    loop {
        let swatch_inner = Instant::now();
        let dat = get_day(day);
        if tsk == 1 { dat.task_1(); } else { dat.task_2(); }
        count += 1;
        times.push(swatch_inner.elapsed());

        let elapsed = swatch.elapsed();
        if (count > 2 && elapsed.as_millis() > 16_000) || (count > 256 && elapsed.as_millis() > 5_000) || count > 2000 {
            break;
        }
    }
    let d = swatch.elapsed();

    times.sort();
    let median = times[times.len()/2];

    println!();
    println!("Day {:0>2} - Task {}: ran {} times for a total duration of {} ms ({:.3} seconds)", day, tsk, count, d.as_millis(), d.as_secs_f32());
    println!("  Average time: {: >5} ms ({:.3} seconds)", d.as_millis() / count, d.as_secs_f32() / count as f32);
    println!("  Median time:  {: >5} ms ({:.3} seconds)", median.as_millis(), median.as_secs_f32());
    println!("  Min time:     {: >5} ms ({:.3} seconds)", times.first().unwrap().as_millis(), times.first().unwrap().as_secs_f32());
    println!("  Max time:     {: >5} ms ({:.3} seconds)", times.last().unwrap().as_millis(), times.last().unwrap().as_secs_f32());
    println!();
}

fn print_table() {
    println!();
    println!("Day | Task 1                   | Task 2                   ");
    println!("----|--------------------------|--------------------------");
    for iday in 1..26 {
        let dat = get_day(iday);

        print!(" {:0>2} |", iday);
        io::stdout().flush().unwrap();

        let r1 = dat.task_1();
        print!(" {: <24} |", r1);
        io::stdout().flush().unwrap();

        let r2 = dat.task_2();
        print!(" {: <24} ", r2);
        io::stdout().flush().unwrap();

        println!();
    }
    println!();
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
            if let Ok(num) = line.trim().parse::<i32>() {
                return num;
            }
        }
    }
}