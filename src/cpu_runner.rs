use std::{io::{stdin, Write}, collections::{HashMap}, fs::File};

use crate::{cpu::Cpu, cpu_helpers::{Instruction, CpuState}};

const HISTORY_SIZE: usize = 1000; 

pub struct CpuRunner {
    pub cpu: Cpu,
    pub op_count: usize,
    pub instruction_history: [Instruction; HISTORY_SIZE],
    pub register_history: [CpuState; HISTORY_SIZE],
    pub pc_traps: HashMap<u16,String>,
    pub continuous_run: bool,
}

// 
impl CpuRunner {
    pub fn new() -> Self{
        CpuRunner { cpu: Cpu::new(), op_count: 0, instruction_history: [Instruction::new(); HISTORY_SIZE], register_history: [CpuState::new(); HISTORY_SIZE], pc_traps: HashMap::new(), continuous_run: false }
    }

    pub fn add_trap(&mut self, loc: u16, message: String) {
        self.pc_traps.insert(loc, message);
    }

    pub fn start_run(&mut self) {
        loop {
            let ni = self.cpu.get_next_instruction();
            self.instruction_history[self.op_count%HISTORY_SIZE] = ni;
            let reg = self.cpu.get_cpu_state();
            self.register_history[self.op_count%HISTORY_SIZE] = reg; 
            
            if self.cpu.pc == self.register_history[(self.op_count as i64 - 1).rem_euclid(HISTORY_SIZE as i64) as usize].pc {
                println!("Found loop!");
                self.continuous_run = false;
            }
            else {
                if self.pc_traps.contains_key(&self.cpu.pc) {
                    println!("Hit trap at pos {:04X}: {}", self.cpu.pc, self.pc_traps[&self.cpu.pc]);
                    self.continuous_run = false;
                }
            }

            if !self.continuous_run {
                self.print_cpu_state();
                println!("Next instruction: {}", ni);
            }

            if !self.continuous_run {
                let should_break = self.handle_input();
                if should_break {
                    break;
                }
            }

            self.op_count += 1;
            self.cpu.execute_next_instruction();
        }
    }

    pub fn print_cpu_state(&self){
        println!("State: {}", self.cpu.get_cpu_state());
    }
    
    pub fn print_instruction(&self, pos: u16){
        println!("{}", self.cpu.get_instruction_at(pos));
    }
    
    pub fn print_history(&self, instruction_amount: u16) {
        let mut counter = if self.op_count > instruction_amount as usize {self.op_count - instruction_amount as usize} else {0};
        while counter <= self.op_count{
            let wrapped_index = counter % HISTORY_SIZE;
            print!("Step {}: {} || ", counter as i64 - self.op_count as i64, self.instruction_history[wrapped_index]);
            println!("Regs: {}", self.register_history[wrapped_index]);
            counter += 1;
        }
    }

    pub fn dump_memory(&self, filename: &str) {
        let mut file = File::create(filename).expect("Could not create file");
        file.write(&self.cpu.memory).expect("Could not write memory to file!");
    }
    
    pub fn print_hex_table(&self, size: usize, start_index: usize) {
        let mut index = start_index;
        // calculate how many spaces to pad before start index
        let padding = start_index % 16;
        // We want to count how many bytes we've printed as opposed to indices
        let mut count = 0;
        // print header
        println!("      00 01 02 03 04 05 06 07 08 09 0A 0B 0C 0D 0E 0F");
        println!("      -----------------------------------------------");
        
        // To add padding at the beginning we copy the amount of padding and count down at the first iteration
        // This also serves to align index to 16 bytes
        let mut padding_align = padding;
        if padding_align != 0 {
            print!("{:04x}: ", index - padding);
            while padding_align > 0 {
                print!("   ");
                padding_align -= 1;
            }
            for _i in padding..16 {
                if count < size {
                    print!("{:02x} ", self.cpu.memory[index]);
                } else {
                    print!("   ");
                }
                count += 1;
                index += 1;
            }
            println!("");
        }
    
        while count < size {
            print!("{:04x}: ", index);
            for _i in 0..16 {    
                if count < size {
                    print!("{:02X} ", self.cpu.memory[index]);
                } else {
                    print!("   ");
                }
                count += 1;
                index += 1;
            }
            println!("");
        }
    }
}

impl CpuRunner {
    fn handle_input(&mut self) -> bool{
        loop {
            let mut cmd = String::new();
            stdin().read_line(&mut cmd).expect("Did not enter a correct string");
            let split_cmd: Vec<&str> = cmd.split(" ").map(|val| val.trim()).collect();
            if split_cmd[0].eq("mem_dec") {
                self.print_mem_dec(split_cmd);
            }
            else if split_cmd[0].eq("mem") {
                self.print_mem_hex(split_cmd);
            }
            else if split_cmd[0].eq("reg") {
                self.print_cpu_state();
            }
            else if split_cmd[0].eq("op") {
                self.print_instruction_cmd(split_cmd);
            }
            else if split_cmd[0].eq("hist") {
                self.print_history_cmd(split_cmd);
            }
            else if split_cmd[0].eq("dump") {
                self.dump_memory(split_cmd[1]);
            }
            else if split_cmd[0].eq("cont") || split_cmd[0].eq("c") {
                self.continuous_run = true;
                return false;
            }
            else if split_cmd[0].eq("next") || split_cmd[0].eq("s") {
                return false;
            }
            else if split_cmd[0].eq("exit") || split_cmd[0].eq("q") {
                return true;
            }
            else{
                println!("No valid command was entered!");
            }
        }
    }

    fn print_instruction_cmd(&self, cmds: Vec<&str>) {
        let pos: u16;
        if cmds.len() < 2 || cmds[1].eq("*") {
            pos = self.cpu.pc;
        }
        else{
            pos = match u16::from_str_radix(cmds[1], 16) {
                Ok(num) => num,
                Err(_) => { 
                    println!("Invalid start value {}", cmds[1]);
                    return;
                },
            };
        }
        self.print_instruction(pos);
    }
    fn print_mem_dec(&self, cmds: Vec<&str>){
        if cmds.len() < 2 {
            println!("Invalid arguments {}", cmds[1]);
            return;
        }
        let start: u16 = match cmds[1].trim().parse() {
            Ok(num) => num,
            Err(_) => { 
                println!("Invalid start value {}", cmds[1]);
                return;
            },
        };
    
        let size: u16 =  match cmds[2].trim().parse() {
            Ok(num) => num,
            Err(_) => { 
                println!("Invalid range value {}", cmds[2]);
                return;
            },
        };
        
        self.print_hex_table(size.into(), start.into());
    }
    
    fn print_mem_hex(&self, cmds: Vec<&str>){
        let start: u16;
        
        if cmds.len() < 3 || cmds[1].eq("*") {
            start = self.cpu.pc;
        }
        else{
            start = match u16::from_str_radix(cmds[1], 16) {
                Ok(num) => num,
                Err(_) => { 
                    println!("Invalid start value {}", cmds[1]);
                    return;
                },
            };
        } 

        let size_str = if cmds.len() < 3 {cmds[1]} else {cmds[2]};

        let size: u16 =  match u16::from_str_radix(size_str, 16) {
            Ok(num) => num,
            Err(_) => { 
                println!("Invalid range value {}", size_str);
                return;
            },
        };
        self.print_hex_table(size.into(), start.into());
    }

    fn print_history_cmd(&self, cmds: Vec<&str>) {
        let size: u16;
        if cmds.len() < 2 || cmds[1].eq("*") {
            size = 10;
        }
        else {
            size = match cmds[1].parse() {
                Ok(num) => num,
                Err(_) => { 
                    println!("Invalid size value {}", cmds[1]);
                    return;
                },
            }
        }

        self.print_history(size);
    }
}