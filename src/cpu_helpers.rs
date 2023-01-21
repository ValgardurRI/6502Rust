use std::fmt;

use strum_macros::{AsRefStr};

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
	AddressMode::Implied, AddressMode::XIndexedIndirect, AddressMode::Invalid, AddressMode::XIndexedIndirect, AddressMode::Zeropage, AddressMode::Zeropage, AddressMode::Zeropage, AddressMode::Zeropage,       // 00
	AddressMode::Implied, AddressMode::Immediate, AddressMode::Accumulator, AddressMode::Immediate, AddressMode::Absolute, AddressMode::Absolute, AddressMode::Absolute, AddressMode::Absolute,                 // 08
	AddressMode::Relative, AddressMode::IndirectYIndex, AddressMode::Invalid, AddressMode::IndirectYIndex, AddressMode::ZeropageX, AddressMode::ZeropageX, AddressMode::ZeropageX, AddressMode::ZeropageX,      // 10
	AddressMode::Implied, AddressMode::AbsoluteY, AddressMode::Implied, AddressMode::AbsoluteY, AddressMode::AbsoluteX, AddressMode::AbsoluteX, AddressMode::AbsoluteX, AddressMode::AbsoluteX,                 // 18
	AddressMode::Absolute, AddressMode::XIndexedIndirect, AddressMode::Invalid, AddressMode::XIndexedIndirect, AddressMode::Zeropage, AddressMode::Zeropage, AddressMode::Zeropage, AddressMode::Zeropage,      // 20
	AddressMode::Implied, AddressMode::Immediate, AddressMode::Accumulator, AddressMode::Immediate, AddressMode::Absolute, AddressMode::Absolute, AddressMode::Absolute, AddressMode::Absolute,                 // 28
	AddressMode::Relative, AddressMode::IndirectYIndex, AddressMode::Invalid, AddressMode::IndirectYIndex, AddressMode::ZeropageX, AddressMode::ZeropageX, AddressMode::ZeropageX, AddressMode::ZeropageX,      // 30
	AddressMode::Implied, AddressMode::AbsoluteY, AddressMode::Implied, AddressMode::AbsoluteY, AddressMode::AbsoluteX, AddressMode::AbsoluteX, AddressMode::AbsoluteX, AddressMode::AbsoluteX,                 // 38
	AddressMode::Implied, AddressMode::XIndexedIndirect, AddressMode::Invalid, AddressMode::XIndexedIndirect, AddressMode::Zeropage, AddressMode::Zeropage, AddressMode::Zeropage, AddressMode::Zeropage,       // 40
	AddressMode::Implied, AddressMode::Immediate, AddressMode::Accumulator, AddressMode::Immediate, AddressMode::Absolute, AddressMode::Absolute, AddressMode::Absolute, AddressMode::Absolute,                 // 48
	AddressMode::Relative, AddressMode::IndirectYIndex, AddressMode::Invalid, AddressMode::IndirectYIndex, AddressMode::ZeropageX, AddressMode::ZeropageX, AddressMode::ZeropageX, AddressMode::ZeropageX,      // 50
	AddressMode::Implied, AddressMode::AbsoluteY, AddressMode::Implied, AddressMode::AbsoluteY, AddressMode::AbsoluteX, AddressMode::AbsoluteX, AddressMode::AbsoluteX, AddressMode::AbsoluteX,                 // 58
	AddressMode::Implied, AddressMode::XIndexedIndirect, AddressMode::Invalid, AddressMode::XIndexedIndirect, AddressMode::Zeropage, AddressMode::Zeropage, AddressMode::Zeropage, AddressMode::Zeropage,       // 60
    AddressMode::Implied, AddressMode::Immediate, AddressMode::Accumulator, AddressMode::Immediate, AddressMode::Indirect, AddressMode::Absolute, AddressMode::Absolute, AddressMode::Absolute,                 // 68
	AddressMode::Relative, AddressMode::IndirectYIndex, AddressMode::Invalid, AddressMode::IndirectYIndex, AddressMode::ZeropageX, AddressMode::ZeropageX, AddressMode::ZeropageX, AddressMode::ZeropageX,      // 70
	AddressMode::Implied, AddressMode::AbsoluteY, AddressMode::Implied, AddressMode::AbsoluteY, AddressMode::AbsoluteX, AddressMode::AbsoluteX, AddressMode::AbsoluteX, AddressMode::AbsoluteX,                 // 78
	AddressMode::Immediate, AddressMode::XIndexedIndirect, AddressMode::Immediate, AddressMode::XIndexedIndirect, AddressMode::Zeropage, AddressMode::Zeropage, AddressMode::Zeropage, AddressMode::Zeropage,   // 80
	AddressMode::Implied, AddressMode::Immediate, AddressMode::Implied, AddressMode::Immediate, AddressMode::Absolute, AddressMode::Absolute, AddressMode::Absolute, AddressMode::Absolute,                     // 88
	AddressMode::Relative, AddressMode::IndirectYIndex, AddressMode::Invalid, AddressMode::IndirectYIndex, AddressMode::ZeropageX, AddressMode::ZeropageX, AddressMode::ZeropageY, AddressMode::ZeropageY,      // 90
	AddressMode::Implied, AddressMode::AbsoluteY, AddressMode::Implied, AddressMode::AbsoluteY, AddressMode::AbsoluteX, AddressMode::AbsoluteX, AddressMode::AbsoluteY, AddressMode::AbsoluteY,                 // 98
	AddressMode::Immediate, AddressMode::XIndexedIndirect, AddressMode::Immediate, AddressMode::XIndexedIndirect, AddressMode::Zeropage, AddressMode::Zeropage, AddressMode::Zeropage, AddressMode::Zeropage,   // A0
	AddressMode::Implied, AddressMode::Immediate, AddressMode::Implied, AddressMode::Immediate, AddressMode::Absolute, AddressMode::Absolute, AddressMode::Absolute, AddressMode::Absolute,                     // A8
	AddressMode::Relative, AddressMode::IndirectYIndex, AddressMode::Invalid, AddressMode::IndirectYIndex, AddressMode::ZeropageX, AddressMode::ZeropageX, AddressMode::ZeropageY, AddressMode::ZeropageY,      // B0
	AddressMode::Implied, AddressMode::AbsoluteY, AddressMode::Implied, AddressMode::AbsoluteY, AddressMode::AbsoluteX, AddressMode::AbsoluteX, AddressMode::AbsoluteY, AddressMode::AbsoluteY,                 // B8
	AddressMode::Immediate, AddressMode::XIndexedIndirect, AddressMode::Immediate, AddressMode::XIndexedIndirect, AddressMode::Zeropage, AddressMode::Zeropage, AddressMode::Zeropage, AddressMode::Zeropage,   // C0
	AddressMode::Implied, AddressMode::Immediate, AddressMode::Implied, AddressMode::Immediate, AddressMode::Absolute, AddressMode::Absolute, AddressMode::Absolute, AddressMode::Absolute,                     // C8
	AddressMode::Relative, AddressMode::IndirectYIndex, AddressMode::Invalid, AddressMode::IndirectYIndex, AddressMode::ZeropageX, AddressMode::ZeropageX, AddressMode::ZeropageX, AddressMode::ZeropageX,      // D0
	AddressMode::Implied, AddressMode::AbsoluteY, AddressMode::Implied, AddressMode::AbsoluteY, AddressMode::AbsoluteX, AddressMode::AbsoluteX, AddressMode::AbsoluteX, AddressMode::AbsoluteX,                 // D8
	AddressMode::Immediate, AddressMode::XIndexedIndirect, AddressMode::Immediate, AddressMode::XIndexedIndirect, AddressMode::Zeropage, AddressMode::Zeropage, AddressMode::Zeropage, AddressMode::Zeropage,   // E0
	AddressMode::Implied, AddressMode::Immediate, AddressMode::Implied, AddressMode::Immediate, AddressMode::Absolute, AddressMode::Absolute, AddressMode::Absolute, AddressMode::Absolute,                     // E8
	AddressMode::Relative, AddressMode::IndirectYIndex, AddressMode::Invalid, AddressMode::IndirectYIndex, AddressMode::ZeropageX, AddressMode::ZeropageX, AddressMode::ZeropageX, AddressMode::ZeropageX,      // F0
	AddressMode::Implied, AddressMode::AbsoluteY, AddressMode::Implied, AddressMode::AbsoluteY, AddressMode::AbsoluteX, AddressMode::AbsoluteX, AddressMode::AbsoluteX, AddressMode::AbsoluteX                  // F8
];

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, AsRefStr)]
pub enum AddressMode{
    Accumulator,
    Absolute,
    AbsoluteX,
    AbsoluteY,
    Immediate,
    Implied,
    Indirect,
    XIndexedIndirect,
    IndirectYIndex,
    Relative,
    Zeropage,
    ZeropageX,
    ZeropageY,
    Invalid
}

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
pub struct Registers {
    pub a: u8,   // Arithmetic register
    pub x: u8,   // X index register
    pub y: u8,   // Y index register
    pub pc: u16, // Program counter
    pub sp: u8,   // Stack pointer
    pub sr: u8,   // Status register
}

impl AddressMode {
    // How many bytes should be read from the instruction list
    pub fn address_size(&self) -> u8 {
        match &self {
            AddressMode::Accumulator => 0,
            AddressMode::Absolute => 2,
            AddressMode::AbsoluteX => 2,
            AddressMode::AbsoluteY => 2,
            AddressMode::Immediate => 1,
            AddressMode::Implied => 0,
            AddressMode::Indirect => 2,
            AddressMode::XIndexedIndirect => 1,
            AddressMode::IndirectYIndex => 1,
            AddressMode::Relative => 1,
            AddressMode::Zeropage => 1,
            AddressMode::ZeropageX => 1,
            AddressMode::ZeropageY => 1,
            AddressMode::Invalid => 0,
        }
    }
}

impl Instruction {
    pub fn new() -> Self {
        Instruction{ operation: Operation::Inv, address_mode: AddressMode::Invalid, value: 0, cycles: 0 }
    }
}

impl Registers {
    pub fn new() -> Self {
        Registers { a: 0, x: 0, y: 0, pc: 0xFFFF, sp: 0, sr: 0 }
    }
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {:04X} #{}", self.operation.as_ref(), self.address_mode.as_ref(), self.value, self.cycles)
    }
}
impl fmt::Display for Registers {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "PC: {:04X}, A: {:02X}, X: {:02X}, Y: {:02X}, SP: {:02X}, SR: {:02X}, NV-BDIZC: {:08b}", self.pc, self.a, self.x, self.y, self.sp, self.sr, self.sr)
    }
}