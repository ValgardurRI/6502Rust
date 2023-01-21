use std::fmt;

use strum_macros::{AsRefStr};

use crate::cpu::Cpu;

// pub const RAM_MIRROR1: usize = 0x0800;
// pub const RAM_MIRROR2: usize = 0x1000;
// pub const RAM_MIRROR3: usize = 0x1800;
// pub const PPU_REGISTERS: usize = 0x2000;
// pub const PPU_REGISTERS_MIRRORS: usize = 0x2008;
// pub const APU_IO_REGISTERS: usize = 0x4000;
// pub const CARTRIDGE_SPACE: usize = 0x4020;
// pub const NMI_VECTOR: u16 = 0xFFFA;
// pub const RESET_VECTOR: u16 = 0xFFFC;
pub const IRQ_VECTOR: u16 = 0xFFFE;

pub const STACK_START: u16 = 0x0100;

pub const FLAG_CARRY: u8        = 0b0000_0001;
pub const FLAG_ZERO: u8         = 0b0000_0010;
pub const FLAG_INTERRUPT: u8    = 0b0000_0100;
pub const FLAG_DECIMAL: u8      = 0b0000_1000;
pub const FLAG_BREAK: u8        = 0b0001_0000;
pub const FLAG_UNUSED: u8       = 0b0010_0000;
pub const FLAG_OVERFLOW: u8     = 0b0100_0000;
pub const FLAG_NEGATIVE: u8     = 0b1000_0000;

pub const OP_CODE_MAP: [Operation; 0x100] = 
[
	Operation::Brk, Operation::Ora, Operation::Inv, Operation::Inv, Operation::Inv, Operation::Ora, Operation::Asl, Operation::Inv,  // 00
	Operation::Php, Operation::Ora, Operation::Asl, Operation::Inv, Operation::Inv, Operation::Ora, Operation::Asl, Operation::Inv,  // 08
	Operation::Bpl, Operation::Ora, Operation::Inv, Operation::Inv, Operation::Inv, Operation::Ora, Operation::Asl, Operation::Inv,  // 10
	Operation::Clc, Operation::Ora, Operation::Inv, Operation::Inv, Operation::Inv, Operation::Ora, Operation::Asl, Operation::Inv,  // 18
	Operation::Jsr, Operation::And, Operation::Inv, Operation::Inv, Operation::Bit, Operation::And, Operation::Rol, Operation::Inv,  // 20
	Operation::Plp, Operation::And, Operation::Rol, Operation::Inv, Operation::Bit, Operation::And, Operation::Rol, Operation::Inv,  // 28
	Operation::Bmi, Operation::And, Operation::Inv, Operation::Inv, Operation::Inv, Operation::And, Operation::Rol, Operation::Inv,  // 30
	Operation::Sec, Operation::And, Operation::Inv, Operation::Inv, Operation::Inv, Operation::And, Operation::Rol, Operation::Inv,  // 38
	Operation::Rti, Operation::Eor, Operation::Inv, Operation::Inv, Operation::Inv, Operation::Eor, Operation::Lsr, Operation::Inv,  // 40
	Operation::Pha, Operation::Eor, Operation::Lsr, Operation::Inv, Operation::Jmp, Operation::Eor, Operation::Lsr, Operation::Inv,  // 48
	Operation::Bvc, Operation::Eor, Operation::Inv, Operation::Inv, Operation::Inv, Operation::Eor, Operation::Lsr, Operation::Inv,  // 50
	Operation::Cli, Operation::Eor, Operation::Inv, Operation::Inv, Operation::Inv, Operation::Eor, Operation::Lsr, Operation::Inv,  // 58
	Operation::Rts, Operation::Adc, Operation::Inv, Operation::Inv, Operation::Inv, Operation::Adc, Operation::Ror, Operation::Inv,  // 60
	Operation::Pla, Operation::Adc, Operation::Ror, Operation::Inv, Operation::Jmp, Operation::Adc, Operation::Ror, Operation::Inv,  // 68
	Operation::Bvs, Operation::Adc, Operation::Inv, Operation::Inv, Operation::Inv, Operation::Adc, Operation::Ror, Operation::Inv,  // 70
	Operation::Sei, Operation::Adc, Operation::Inv, Operation::Inv, Operation::Inv, Operation::Adc, Operation::Ror, Operation::Inv,  // 78
	Operation::Inv, Operation::Sta, Operation::Inv, Operation::Inv, Operation::Sty, Operation::Sta, Operation::Stx, Operation::Inv,  // 80
	Operation::Dey, Operation::Inv, Operation::Txa, Operation::Inv, Operation::Sty, Operation::Sta, Operation::Stx, Operation::Inv,  // 88
	Operation::Bcc, Operation::Sta, Operation::Inv, Operation::Inv, Operation::Sty, Operation::Sta, Operation::Stx, Operation::Inv,  // 90
	Operation::Tya, Operation::Sta, Operation::Txs, Operation::Inv, Operation::Inv, Operation::Sta, Operation::Inv, Operation::Inv,  // 98
	Operation::Ldy, Operation::Lda, Operation::Ldx, Operation::Inv, Operation::Ldy, Operation::Lda, Operation::Ldx, Operation::Inv,  // A0
	Operation::Tay, Operation::Lda, Operation::Tax, Operation::Inv, Operation::Ldy, Operation::Lda, Operation::Ldx, Operation::Inv,  // A8
	Operation::Bcs, Operation::Lda, Operation::Inv, Operation::Inv, Operation::Ldy, Operation::Lda, Operation::Ldx, Operation::Inv,  // B0
	Operation::Clv, Operation::Lda, Operation::Tsx, Operation::Inv, Operation::Ldy, Operation::Lda, Operation::Ldx, Operation::Inv,  // B8
	Operation::Cpy, Operation::Cmp, Operation::Inv, Operation::Inv, Operation::Cpy, Operation::Cmp, Operation::Dec, Operation::Inv,  // C0
	Operation::Iny, Operation::Cmp, Operation::Dex, Operation::Inv, Operation::Cpy, Operation::Cmp, Operation::Dec, Operation::Inv,  // C8
	Operation::Bne, Operation::Cmp, Operation::Inv, Operation::Inv, Operation::Inv, Operation::Cmp, Operation::Dec, Operation::Inv,  // D0
	Operation::Cld, Operation::Cmp, Operation::Inv, Operation::Inv, Operation::Inv, Operation::Cmp, Operation::Dec, Operation::Inv,  // D8
	Operation::Cpx, Operation::Sbc, Operation::Inv, Operation::Inv, Operation::Cpx, Operation::Sbc, Operation::Inc, Operation::Inv,  // E0
	Operation::Inx, Operation::Sbc, Operation::Nop, Operation::Inv, Operation::Cpx, Operation::Sbc, Operation::Inc, Operation::Inv,  // E8
	Operation::Beq, Operation::Sbc, Operation::Inv, Operation::Inv, Operation::Inv, Operation::Sbc, Operation::Inc, Operation::Inv,  // F0
	Operation::Sed, Operation::Sbc, Operation::Inv, Operation::Inv, Operation::Inv, Operation::Sbc, Operation::Inc, Operation::Inv   // F8
];

pub const ADDRESS_MODE_MAP: [AddressMode; 0x100] =
[
	AddressMode::Imp, AddressMode::Inx, AddressMode::Inv, AddressMode::Inx, AddressMode::Zpg, AddressMode::Zpg, AddressMode::Zpg, AddressMode::Zpg, // 00
	AddressMode::Imp, AddressMode::Imm, AddressMode::Acc, AddressMode::Imm, AddressMode::Abs, AddressMode::Abs, AddressMode::Abs, AddressMode::Abs, // 08
	AddressMode::Rel, AddressMode::Iny, AddressMode::Inv, AddressMode::Iny, AddressMode::Zpx, AddressMode::Zpx, AddressMode::Zpx, AddressMode::Zpx, // 10
	AddressMode::Imp, AddressMode::Aby, AddressMode::Imp, AddressMode::Aby, AddressMode::Abx, AddressMode::Abx, AddressMode::Abx, AddressMode::Abx, // 18
	AddressMode::Abs, AddressMode::Inx, AddressMode::Inv, AddressMode::Inx, AddressMode::Zpg, AddressMode::Zpg, AddressMode::Zpg, AddressMode::Zpg, // 20
	AddressMode::Imp, AddressMode::Imm, AddressMode::Acc, AddressMode::Imm, AddressMode::Abs, AddressMode::Abs, AddressMode::Abs, AddressMode::Abs, // 28
	AddressMode::Rel, AddressMode::Iny, AddressMode::Inv, AddressMode::Iny, AddressMode::Zpx, AddressMode::Zpx, AddressMode::Zpx, AddressMode::Zpx, // 30
	AddressMode::Imp, AddressMode::Aby, AddressMode::Imp, AddressMode::Aby, AddressMode::Abx, AddressMode::Abx, AddressMode::Abx, AddressMode::Abx, // 38
	AddressMode::Imp, AddressMode::Inx, AddressMode::Inv, AddressMode::Inx, AddressMode::Zpg, AddressMode::Zpg, AddressMode::Zpg, AddressMode::Zpg, // 40
	AddressMode::Imp, AddressMode::Imm, AddressMode::Acc, AddressMode::Imm, AddressMode::Abs, AddressMode::Abs, AddressMode::Abs, AddressMode::Abs, // 48
	AddressMode::Rel, AddressMode::Iny, AddressMode::Inv, AddressMode::Iny, AddressMode::Zpx, AddressMode::Zpx, AddressMode::Zpx, AddressMode::Zpx, // 50
	AddressMode::Imp, AddressMode::Aby, AddressMode::Imp, AddressMode::Aby, AddressMode::Abx, AddressMode::Abx, AddressMode::Abx, AddressMode::Abx, // 58
	AddressMode::Imp, AddressMode::Inx, AddressMode::Inv, AddressMode::Inx, AddressMode::Zpg, AddressMode::Zpg, AddressMode::Zpg, AddressMode::Zpg, // 60
    AddressMode::Imp, AddressMode::Imm, AddressMode::Acc, AddressMode::Imm, AddressMode::Ind, AddressMode::Abs, AddressMode::Abs, AddressMode::Abs, // 68
	AddressMode::Rel, AddressMode::Iny, AddressMode::Inv, AddressMode::Iny, AddressMode::Zpx, AddressMode::Zpx, AddressMode::Zpx, AddressMode::Zpx, // 70
	AddressMode::Imp, AddressMode::Aby, AddressMode::Imp, AddressMode::Aby, AddressMode::Abx, AddressMode::Abx, AddressMode::Abx, AddressMode::Abx, // 78
	AddressMode::Imm, AddressMode::Inx, AddressMode::Imm, AddressMode::Inx, AddressMode::Zpg, AddressMode::Zpg, AddressMode::Zpg, AddressMode::Zpg, // 80
	AddressMode::Imp, AddressMode::Imm, AddressMode::Imp, AddressMode::Imm, AddressMode::Abs, AddressMode::Abs, AddressMode::Abs, AddressMode::Abs, // 88
	AddressMode::Rel, AddressMode::Iny, AddressMode::Inv, AddressMode::Iny, AddressMode::Zpx, AddressMode::Zpx, AddressMode::Zpy, AddressMode::Zpy, // 90
	AddressMode::Imp, AddressMode::Aby, AddressMode::Imp, AddressMode::Aby, AddressMode::Abx, AddressMode::Abx, AddressMode::Aby, AddressMode::Aby, // 98
	AddressMode::Imm, AddressMode::Inx, AddressMode::Imm, AddressMode::Inx, AddressMode::Zpg, AddressMode::Zpg, AddressMode::Zpg, AddressMode::Zpg, // A0
	AddressMode::Imp, AddressMode::Imm, AddressMode::Imp, AddressMode::Imm, AddressMode::Abs, AddressMode::Abs, AddressMode::Abs, AddressMode::Abs, // A8
	AddressMode::Rel, AddressMode::Iny, AddressMode::Inv, AddressMode::Iny, AddressMode::Zpx, AddressMode::Zpx, AddressMode::Zpy, AddressMode::Zpy, // B0
	AddressMode::Imp, AddressMode::Aby, AddressMode::Imp, AddressMode::Aby, AddressMode::Abx, AddressMode::Abx, AddressMode::Aby, AddressMode::Aby, // B8
	AddressMode::Imm, AddressMode::Inx, AddressMode::Imm, AddressMode::Inx, AddressMode::Zpg, AddressMode::Zpg, AddressMode::Zpg, AddressMode::Zpg, // C0
	AddressMode::Imp, AddressMode::Imm, AddressMode::Imp, AddressMode::Imm, AddressMode::Abs, AddressMode::Abs, AddressMode::Abs, AddressMode::Abs, // C8
	AddressMode::Rel, AddressMode::Iny, AddressMode::Inv, AddressMode::Iny, AddressMode::Zpx, AddressMode::Zpx, AddressMode::Zpx, AddressMode::Zpx, // D0
	AddressMode::Imp, AddressMode::Aby, AddressMode::Imp, AddressMode::Aby, AddressMode::Abx, AddressMode::Abx, AddressMode::Abx, AddressMode::Abx, // D8
	AddressMode::Imm, AddressMode::Inx, AddressMode::Imm, AddressMode::Inx, AddressMode::Zpg, AddressMode::Zpg, AddressMode::Zpg, AddressMode::Zpg, // E0
	AddressMode::Imp, AddressMode::Imm, AddressMode::Imp, AddressMode::Imm, AddressMode::Abs, AddressMode::Abs, AddressMode::Abs, AddressMode::Abs, // E8
	AddressMode::Rel, AddressMode::Iny, AddressMode::Inv, AddressMode::Iny, AddressMode::Zpx, AddressMode::Zpx, AddressMode::Zpx, AddressMode::Zpx, // F0
	AddressMode::Imp, AddressMode::Aby, AddressMode::Imp, AddressMode::Aby, AddressMode::Abx, AddressMode::Abx, AddressMode::Abx, AddressMode::Abx  // F8
];

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, AsRefStr)]
pub enum AddressMode{
    Acc,
    Abs,
    Abx,
    Aby,
    Imm,
    Imp,
    Ind,
    Inx,
    Iny,
    Rel,
    Zpg,
    Zpx,
    Zpy,
    Inv
}

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, AsRefStr)]
pub enum Operation{
    Adc,
    And,
    Asl,
    Bcc,
    Bcs,
    Beq,
    Bit,
    Bmi,
    Bne,
    Bpl,
    Brk,
    Bvc,
    Bvs,
    Clc,
    Cld,
    Cli,
    Clv,
    Cmp,
    Cpx,
    Cpy,
    Dec,
    Dex,
    Dey,
    Eor,
    Inc,
    Inx,
    Iny,
    Jmp,
    Jsr,
    Lda,
    Ldx,
    Ldy,
    Lsr,
    Nop,
    Ora,
    Pha,
    Php,
    Pla,
    Plp,
    Rol,
    Ror,
    Rti,
    Rts,
    Sbc,
    Sec,
    Sed,
    Sei,
    Sta,
    Stx,
    Sty,
    Tax,
    Tay,
    Tsx,
    Txa,
    Txs,
    Tya,
    Inv
}

pub enum CycleType {
    SetValue1,          // Imm: 2, Zpg: 3, Zpx: 4, Abs: 4, Abx: 4*, Aby: 4*, Inx: 6, Iny: 5*
    SetValue2,          // Acc: 2, Zpg: 5, Zpx: 6, Abs: 6, Abx: 7
    Branch,             // Rel: 2*
    Base,               // Imp: 2
    Jump,               // Abs: 3, Ind: 5
    JumpSubroutine,     // Abs: 6
    Break,              // Imp: 7
    PushStack,          // Imp: 3
    PullStack,          // Imp: 4
    Return,             // Imp: 6
    StoreAccumulator    // Imm: 2, Zpg: 3, Zpx: 4, Abs: 4, Abx: 5, Aby: 5, Inx: 6, Iny: 6  // This is like SetValue1 but assumes worst case for all conditional cycles 
}

#[derive(Clone, Copy)]
pub struct Operand{
    pub value: u16, 
    pub address: u16
}

#[derive(Clone, Copy)]
pub struct Instruction { 
    pub operation: Operation, 
    pub address_mode: AddressMode,
    pub value: u16, 
    pub cycles: u8 
}

#[derive(Clone, Copy)]
pub struct CpuState {
    pub a: u8,       // Arithmetic register
    pub x: u8,       // X index register
    pub y: u8,       // Y index register
    pub pc: u16,     // Program counter
    pub sp: u8,      // Stack pointer
    pub sr: u8,      // Status register
    pub cycles: u64,
}

impl AddressMode {
    // How many bytes should be read from the instruction list
    pub fn address_size(&self) -> u8 {
        match &self {
            AddressMode::Acc => 0,
            AddressMode::Abs => 2,
            AddressMode::Abx => 2,
            AddressMode::Aby => 2,
            AddressMode::Imm => 1,
            AddressMode::Imp => 0,
            AddressMode::Ind => 2,
            AddressMode::Inx => 1,
            AddressMode::Iny => 1,
            AddressMode::Rel => 1,
            AddressMode::Zpg => 1,
            AddressMode::Zpx => 1,
            AddressMode::Zpy => 1,
            AddressMode::Inv => 0,
        }
    }
}

impl Instruction {
    pub fn new() -> Self {
        Instruction{ operation: Operation::Inv, address_mode: AddressMode::Inv, value: 0, cycles: 0 }
    }
}

impl CpuState {
    pub fn new() -> Self {
        CpuState { a: 0, x: 0, y: 0, pc: 0xFFFF, sp: 0, sr: 0, cycles: 0 }
    }
}

impl CycleType {
    pub fn GetCycleCount(&self, mode: &AddressMode, address: u16, register: u8) -> u8 {
        match self {
            CycleType::SetValue1 => match mode {
                AddressMode::Imm => 2,
                AddressMode::Zpg => 3,
                AddressMode::Zpx => 4,
                AddressMode::Zpy => 4,
                AddressMode::Abs => 4,
                AddressMode::Abx => 4 + if (address + register as u16) & 0xFF00 != address & 0xFF00 {1} else {0}, // add 1 cycle if page break
                AddressMode::Aby => 4 + if (address + register as u16) & 0xFF00 != address & 0xFF00 {1} else {0}, // add 1 cycle if page break
                AddressMode::Inx => 6,
                AddressMode::Iny => 5 + if (address&0xFF) + register as u16 > 0xFF {1} else {0}, // add 1 cycle if page break
                _ => panic!("Invalid address mode for cycle type")
            },
            CycleType::SetValue2 => match mode {
                AddressMode::Acc => 2,
                AddressMode::Zpg => 5,
                AddressMode::Zpx => 6,
                AddressMode::Abs => 6,
                AddressMode::Abx => 7,
                _ => panic!("Invalid address mode for cycle type")
            },
            CycleType::Branch => 3 + if (address.wrapping_add((register as i8) as u16)) & 0xFF00 != address & 0xFF00 {1} else {0}, // add 1 cycle if page break. NOTE this is only valid if we branch, 2 if not branched
            CycleType::Base => 2,
            CycleType::Jump => match mode {
                AddressMode::Abs => 3,
                AddressMode::Ind => 5,
                _ => panic!("Invalid address mode for cycle type")
            },
            CycleType::JumpSubroutine => 6,
            CycleType::Break => 7,
            CycleType::PushStack => 3,
            CycleType::PullStack => 4,
            CycleType::Return => 6,
            CycleType::StoreAccumulator => match mode {
                AddressMode::Imm => 2,
                AddressMode::Zpg => 3,
                AddressMode::Zpx => 4,
                AddressMode::Abs => 4,
                AddressMode::Abx => 5,
                AddressMode::Aby => 5,
                AddressMode::Inx => 6,
                AddressMode::Iny => 6,
                _ => panic!("Invalid address mode for cycle type")
            },
        }
    }
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {:04X} #{}", self.operation.as_ref(), self.address_mode.as_ref(), self.value, self.cycles)
    }
}
impl fmt::Display for CpuState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "PC: {:04X}, A: {:02X}, X: {:02X}, Y: {:02X}, SP: {:02X}, SR: {:02X}, NV-BDIZC: {:08b}, cycles: {}", self.pc, self.a, self.x, self.y, self.sp, self.sr, self.sr, self.cycles)
    }
}