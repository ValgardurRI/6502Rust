use std::time::Instant;
use std::{fs::read, io::Write};

use crate::cpu_runner::CpuRunner;

mod cpu_helpers;
mod cpu;
mod bcd;
mod cpu_runner;

fn main() {

    // ctrlc::set_handler(|| {
    //     unsafe{
    //         interrupt = true;
    //     }
    //     println!("received Ctrl+C!");
    // })
    // .expect("Error setting Ctrl-C handler");


    
    let mut runner = CpuRunner::new();
    
    let data = read("./test/6502_functional_test.bin").expect("could not read test file");
    let mut memory_slice = &mut runner.cpu.memory[0x000a..=0xFFFF];
    memory_slice.write(&data).expect("Could not write to 6502 memory");
    runner.cpu.pc = 0x400;
    
    
    // let data = read("./test/6502_decimal_test.bin").expect("could not read test file");
    // let mut memory_slice = &mut runner.cpu.memory[0x0200..=0x02f9];
    // memory_slice.write(&data).expect("Could not write to 6502 memory");
    // runner.cpu.pc = 0x200;
    runner.continuous_run = true;
    
    let start = Instant::now();

    // runner.add_trap(0x0444, "test0: relative BEQ addressing testing".into());
    // runner.add_trap(0x0594, "test1: partial test BNE & CMP, CPX, CPY immediate".into());
    // runner.add_trap(0x05d4, "test2: stack operations PHA PHP PLA PLP".into());
    // runner.add_trap(0x0608, "test3: branch decisions BPL BMI BVC BVS BCC BCS BNE BEQ".into());
    // runner.add_trap(0x0782, "test4: PHA does not alter flags or accumulator but PLA does".into());
    // runner.add_trap(0x087e, "test5: partial pretest EOR #".into());
    // runner.add_trap(0x08b2, "test6: start non-branch pc modifiers".into());
    // runner.add_trap(0x08fc, "test7: jump absolute".into());
    // runner.add_trap(0x0952, "test8: jump indirect".into());
    // runner.add_trap(0x098e, "test9: jump subroutine & return from subroutine".into());
    // runner.add_trap(0x09c5, "test10: break & return from interrupt".into());
    // runner.add_trap(0x0a1d, "test11: test set and clear flags CLC CLI CLD CLV SEC SEI SED".into());
    // runner.add_trap(0x0ac3, "test12: testing index register increment/decrement and transfer INX INY DEX DEY TAX TXA TAY TYA".into());
    // runner.add_trap(0x0d89, "test13: SX sets NZ - TXS does not. This section also tests for proper stack wrap around.".into());
    // runner.add_trap(0x0e52, "test14: testing index register load & store LDY LDX STY STX all addressing modes. LDX / STX - zp,y / abs,y".into());
    // runner.add_trap(0x0f0d, "test15: indexed wraparound test (only zp should wrap)".into());
    // runner.add_trap(0x0f4f, "test16: LDY / STY - zp,x / abs,x".into());
    // runner.add_trap(0x1006, "test17: indexed wraparound test (only zp should wrap)".into());
    // runner.add_trap(0x1046, "test18: LDX / STX - zp / abs / #".into());
    // runner.add_trap(0x133c, "test19: LDY / STY - zp / abs / #".into());
    // runner.add_trap(0x1636, "test20: testing load / store accumulator LDA / STA all addressing modes. LDA / STA - zp,x / abs,x".into());
    // runner.add_trap(0x16e7, "test21: LDA / STA - (zp),y / abs,y / (zp,x)".into());
    // runner.add_trap(0x1802, "test22: indexed wraparound test (only zp should wrap)".into());
    // runner.add_trap(0x18a5, "test23: LDA / STA - zp / abs / #".into());
    // runner.add_trap(0x1b6f, "test24: testing bit test & compares BIT CPX CPY CMP all addressing modes. BIT - zp / abs".into());
    // runner.add_trap(0x1cc3, "test25: CPX - zp / abs / # ".into());
    // runner.add_trap(0x1dd1, "test26: CPY - zp / abs / #   ".into());
    // runner.add_trap(0x1edf, "test27: CMP - zp / abs / #  ".into());
    // runner.add_trap(0x22c3, "test28: testing shifts - ASL LSR ROL ROR all addressing modes. shifts - accumulator".into());
    // runner.add_trap(0x2407, "test29: shifts - zeropage".into());
    // runner.add_trap(0x2587, "test30: shifts - absolute".into());
    // runner.add_trap(0x272b, "test31: shifts - zp indexed".into());
    // runner.add_trap(0x28ab, "test32: shifts - abs indexed".into());
    // runner.add_trap(0x2a4f, "test33: testing memory increment/decrement - INC DEC all addressing modes".into());
    // runner.add_trap(0x2af9, "test34: absolute memory".into());
    // runner.add_trap(0x2bb3, "test35: zeropage indexed".into());
    // runner.add_trap(0x2c61, "test36: memory indexed".into());
    // runner.add_trap(0x2d1f, "test37: AND".into());
    // runner.add_trap(0x2f17, "test38: EOR".into());
    // runner.add_trap(0x310f, "test39: OR".into());
    // runner.add_trap(0x3308, "test40: full binary add/subtract test".into());
    // runner.add_trap(0x336d, "test41: decimal add/subtract test".into());
    // runner.add_trap(0x3411, "test42: decimal/binary switch test. tests CLD, SED, PLP, RTI to properly switch between decimal & binary opcode".into());
    //runner.add_trap(0x35CF, "debug".into());
    
    //runner.add_trap(0x3469, "Success!".into());
    runner.start_run();

    let elapsed = start.elapsed();
    println!("Run finished! Operations: {}, mem4: {}, time elapsed: {:?}, instruction Hz: {:?}", runner.op_count, runner.cpu.memory[4], elapsed.as_millis(), (runner.op_count as f64)/(elapsed.as_secs_f64()));
    runner.print_registers();
    runner.print_hex_table(0x20, 0);
}