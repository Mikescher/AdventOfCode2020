use crate::common::AdventOfCodeDay;

use itertools::Itertools;
use std::collections::HashSet;
use std::fmt::Display;

#[derive(Debug)]
struct HGCProgramm {
    prog: Vec<HGCCommand>,
}

impl HGCProgramm {
    fn parse_program(code: String) -> HGCProgramm {
        HGCProgramm {
            prog: code.lines().map(HGCCommand::parse).collect(),
        }
    }
}

#[derive(Debug)]
struct HGCCommand {
    cmd: HGCCommandType,
    arg: i32,
}

impl HGCCommand {
    fn parse(line: &str) -> HGCCommand {
        let (str_cmd, str_arg) =  line.split(' ').collect_tuple::<(_, _)>().unwrap();
        HGCCommand {
            cmd: HGCCommandType::parse(str_cmd),
            arg: str_arg.parse::<i32>().unwrap(),
        }
    }
}

#[derive(Debug, PartialEq)]
enum HGCCommandType {
    NOP,
    ACC,
    JMP,
}

impl HGCCommandType {
    fn parse(val: &str) -> HGCCommandType {
        match val {
            "acc" => HGCCommandType::ACC,
            "jmp" => HGCCommandType::JMP,
            "nop" => HGCCommandType::NOP,
            _ => panic!("not a cmd")
        }
    }
}

impl Display for HGCCommandType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug)]
struct HGCMachineState {
    trace: bool,

    acc: i32,
    ip: usize,
}

impl HGCMachineState {
    fn new() -> HGCMachineState {
        HGCMachineState {
            trace: false,

            acc: 0,
            ip: 0,
        }
    }

    fn exec_step(&mut self, prog: &HGCProgramm) {
        self.exec_single(&prog.prog[self.ip])
    }

    fn exec_single(&mut self, cmd: &HGCCommand) {
        if self.trace {
            println!("ip: {: <4}  ||  cmd:[{}({: <5})]  (acc: {})", self.ip, cmd.cmd, cmd.arg, self.acc)
        }

        if cmd.cmd == HGCCommandType::NOP {
            self.ip += 1;
        } else if cmd.cmd == HGCCommandType::JMP {
            self.ip = (self.ip as i32 + cmd.arg) as  usize;
        } else if cmd.cmd == HGCCommandType::ACC {
            self.acc += cmd.arg;
            self.ip += 1;
        }
    }
}

pub struct Day08 {
    input: HGCProgramm,
}

impl Day08 {
    pub fn new() -> Self {
        let input_bytes = include_bytes!("../res/08_input.txt");
        let input_str = String::from_utf8_lossy(input_bytes);
        
        Self {
            input: HGCProgramm::parse_program(input_str.to_owned().to_string())
        }
    }
}

impl AdventOfCodeDay for Day08 {

    fn task_1(&self) -> String {
        let mut vm = HGCMachineState::new();

        //vm.trace = true;

        let mut visited: HashSet<usize> = HashSet::new();
        visited.insert(vm.ip);

        loop {
            vm.exec_step(&self.input);
            if visited.contains(&vm.ip) {
                return vm.acc.to_string();
            }
            visited.insert(vm.ip);
        }
    }

    fn task_2(&self) -> String  {
        return "TODO".to_owned() //TODO
    }
}