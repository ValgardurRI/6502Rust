use byteorder::{ByteOrder, LittleEndian};
use crate::{bcd::BcdOps, cpu_helpers::{Instruction, AddressMode, Operand, FLAG_CARRY, FLAG_DECIMAL, FLAG_OVERFLOW, FLAG_NEGATIVE, FLAG_ZERO, STACK_START, FLAG_INTERRUPT, IRQ_VECTOR, FLAG_BREAK, FLAG_UNUSED, OP_CODE_MAP, ADDRESS_MODE_MAP, Registers}};

#[macro_export]
macro_rules! stack_index {
    ($x:expr) => {
        ($x as u16 + STACK_START)
    };
}

pub struct Cpu {
    pub a: u8,   // Arithmetic register
    pub x: u8,   // X index register
    pub y: u8,   // Y index register
    pub pc: u16, // Program counter
    pub sp: u8,   // Stack pointer
    pub sr: u8,   // Status register

    pub memory: [u8; 0x10000]
}

impl Cpu {
    pub fn new() -> Cpu{
        Cpu {
            a: 0,
            x: 0,
            y: 0,
            pc: 0,
            sp: 0xff,
            sr: 0b0011_0100,

            memory: [0; 0x10000]
        }
    }

    pub fn write_memory_u8(&mut self, ind: u16, val: u8) {
        self.memory[ind as usize] = val;
    }

    pub fn write_memory_u16(&mut self, ind: u16, val: u16) {
        self.memory[ind as usize] = ((val & 0xFF00) >> 4) as u8;
        let buf = &mut self.memory[(ind - 1) as usize ..= ind as usize];
        LittleEndian::write_u16(buf, val);
    }

    pub fn read_memory_u8(&self, ind: u16) -> u8{
        self.memory[ind as usize]
    }

    pub fn read_memory_u16(&self, ind: u16) -> u16{
        let uind = ind as usize;
        let indirect = false;
        if indirect {
            // In 6502, the first byte is not carried, so if uind is 0x12FF, the low byte of 0x12FF and high byte of 0x1200 is read instead of 0x1300's high byte.
            // TODO: apparently this is only the case with indirect referencing
            let uind2 = (uind & 0x00FF) + (uind as u8).wrapping_add(1) as usize;
            let bytes = [self.memory[uind], self.memory[uind2]];
            LittleEndian::read_u16(&bytes)
        }
        else{
            let bytes = &self.memory[uind..=uind+1];
            LittleEndian::read_u16(&bytes)
        }
    }

    pub fn get_flag(&self, flag: u8) -> bool {
        (self.sr & flag as u8) != 0
    }

    pub fn set_flag(&mut self, value: bool, flag: u8) {
        if value {
            self.sr |= flag as u8;
        }
        else {
            self.sr &= !(flag as u8);
        }
    }

    pub fn get_register_state(&self) -> Registers {
        Registers{a: self.a, x: self.x, y: self.y, pc: self.pc, sp: self.sp, sr: self.sr}
    }

    pub fn get_next_instruction(&self) -> Instruction {
        self.get_instruction_at(self.pc)
    }

    pub fn get_instruction_at(&self, pos: u16) -> Instruction {
        let instr = self.memory[pos as usize];
        let op = OP_CODE_MAP[instr as usize];
        let mode = ADDRESS_MODE_MAP[instr as usize];
        let value: u16 ;
        let address_size = mode.address_size();
        if address_size == 2 {
            value = self.read_memory_u16(self.pc + 1);
        }
        else if address_size == 1 {
            value = self.read_memory_u8(self.pc + 1) as u16;
        }
        else {
            value = 0;
        }
        Instruction { operation: op, address_mode: mode, value: value, cycles: 0 }
    }

    /// Execute the instruction specified via the program counter.
    /// Returns the clock cycles required for this instruction.
    /// See https://www.masswerk.at/6502/6502_instruction_set.html#ADC
    pub fn execute_next_instruction(&mut self) -> u8 {
        
        let instr = self.memory[self.pc as usize];
        self.pc += 1;

        let cycles = match instr {
            0x00        => self.brk(),
            0x01        => self.ora(AddressMode::XIndexedIndirect),
            0x02..=0x04 => self.inv(), 
            0x05        => self.ora(AddressMode::Zeropage),
            0x06        => self.asl(AddressMode::Zeropage),
            0x07        => self.inv(),
            0x08        => self.php(),
            0x09        => self.ora(AddressMode::Immediate),
            0x0A        => self.asl(AddressMode::Accumulator),
            0x0B..=0x0C => self.inv(),
            0x0D        => self.ora(AddressMode::Absolute),
            0x0E        => self.asl(AddressMode::Absolute),
            0x0F        => self.inv(),
            
            0x10        => self.bpl(AddressMode::Relative),
            0x11        => self.ora(AddressMode::IndirectYIndex),
            0x12..=0x14 => self.inv(), 
            0x15        => self.ora(AddressMode::ZeropageX),
            0x16        => self.asl(AddressMode::ZeropageX),
            0x17        => self.inv(),
            0x18        => self.clc(),
            0x19        => self.ora(AddressMode::AbsoluteY),
            0x1A..=0x1C => self.inv(),
            0x1D        => self.ora(AddressMode::AbsoluteX),
            0x1E        => self.asl(AddressMode::AbsoluteX),
            0x1F        => self.inv(),
            
            0x20        => self.jsr(AddressMode::Absolute),
            0x21        => self.and(AddressMode::XIndexedIndirect),
            0x22..=0x23 => self.inv(), 
            0x24        => self.bit(AddressMode::Zeropage),
            0x25        => self.and(AddressMode::Zeropage),
            0x26        => self.rol(AddressMode::Zeropage),
            0x27        => self.inv(),
            0x28        => self.plp(),
            0x29        => self.and(AddressMode::Immediate),
            0x2A        => self.rol(AddressMode::Accumulator),
            0x2B        => self.inv(),
            0x2C        => self.bit(AddressMode::Absolute),
            0x2D        => self.and(AddressMode::Absolute),
            0x2E        => self.rol(AddressMode::Absolute),
            0x2F        => self.inv(),

            0x30        => self.bmi(AddressMode::Relative),
            0x31        => self.and(AddressMode::IndirectYIndex),
            0x32..=0x34 => self.inv(), 
            0x35        => self.and(AddressMode::ZeropageX),
            0x36        => self.rol(AddressMode::ZeropageX),
            0x37        => self.inv(),
            0x38        => self.sec(),
            0x39        => self.and(AddressMode::AbsoluteY),
            0x3A..=0x3C => self.inv(),
            0x3D        => self.and(AddressMode::AbsoluteX),
            0x3E        => self.rol(AddressMode::AbsoluteX),
            0x3F        => self.inv(),

            0x40        => self.rti(),
            0x41        => self.eor(AddressMode::XIndexedIndirect),
            0x42..=0x44 => self.inv(), 
            0x45        => self.eor(AddressMode::Zeropage),
            0x46        => self.lsr(AddressMode::Zeropage),
            0x47        => self.inv(),
            0x48        => self.pha(),
            0x49        => self.eor(AddressMode::Immediate),
            0x4A        => self.lsr(AddressMode::Accumulator),
            0x4B        => self.inv(),
            0x4C        => self.jmp(AddressMode::Absolute),
            0x4D        => self.eor(AddressMode::Absolute),
            0x4E        => self.lsr(AddressMode::Absolute),
            0x4F        => self.inv(),
            
            0x50        => self.bvc(AddressMode::Relative),
            0x51        => self.eor(AddressMode::IndirectYIndex),
            0x52..=0x54 => self.inv(), 
            0x55        => self.eor(AddressMode::ZeropageX),
            0x56        => self.lsr(AddressMode::ZeropageX),
            0x57        => self.inv(),
            0x58        => self.cli(),
            0x59        => self.eor(AddressMode::AbsoluteY),
            0x5A..=0x5C => self.inv(),
            0x5D        => self.eor(AddressMode::AbsoluteX),
            0x5E        => self.lsr(AddressMode::AbsoluteX),
            0x5F        => self.inv(),
            
            0x60        => self.rts(),
            0x61        => self.adc(AddressMode::XIndexedIndirect),
            0x62..=0x64 => self.inv(), 
            0x65        => self.adc(AddressMode::Zeropage),
            0x66        => self.ror(AddressMode::Zeropage),
            0x67        => self.inv(),
            0x68        => self.pla(),
            0x69        => self.adc(AddressMode::Immediate),
            0x6A        => self.ror(AddressMode::Accumulator),
            0x6B        => self.inv(),
            0x6C        => self.jmp(AddressMode::Indirect),
            0x6D        => self.adc(AddressMode::Absolute),
            0x6E        => self.ror(AddressMode::Absolute),
            0x6F        => self.inv(),
            
            0x70        => self.bvs(AddressMode::Relative),
            0x71        => self.adc(AddressMode::IndirectYIndex),
            0x72..=0x74 => self.inv(), 
            0x75        => self.adc(AddressMode::ZeropageX),
            0x76        => self.ror(AddressMode::ZeropageX),
            0x77        => self.inv(),
            0x78        => self.sei(),
            0x79        => self.adc(AddressMode::AbsoluteY),
            0x7A..=0x7C => self.inv(),
            0x7D        => self.adc(AddressMode::AbsoluteX),
            0x7E        => self.ror(AddressMode::AbsoluteX),
            0x7F        => self.inv(),
            
            0x80        => self.inv(),
            0x81        => self.sta(AddressMode::XIndexedIndirect),
            0x82..=0x83 => self.inv(), 
            0x84        => self.sty(AddressMode::Zeropage),
            0x85        => self.sta(AddressMode::Zeropage),
            0x86        => self.stx(AddressMode::Zeropage),
            0x87        => self.inv(),
            0x88        => self.dey(),
            0x89        => self.inv(),
            0x8A        => self.txa(),
            0x8B        => self.inv(),
            0x8C        => self.sty(AddressMode::Absolute),
            0x8D        => self.sta(AddressMode::Absolute),
            0x8E        => self.stx(AddressMode::Absolute),
            0x8F        => self.inv(),
            
            0x90        => self.bcc(AddressMode::Relative),
            0x91        => self.sta(AddressMode::IndirectYIndex),
            0x92..=0x93 => self.inv(), 
            0x94        => self.sty(AddressMode::ZeropageX),
            0x95        => self.sta(AddressMode::ZeropageX),
            0x96        => self.stx(AddressMode::ZeropageY),
            0x97        => self.inv(),
            0x98        => self.tya(),
            0x99        => self.sta(AddressMode::AbsoluteY),
            0x9A        => self.txs(),
            0x9B..=0x9C => self.inv(),
            0x9D        => self.sta(AddressMode::AbsoluteX),
            0x9E..=0x9F => self.inv(),

            0xA0        => self.ldy(AddressMode::Immediate),
            0xA1        => self.lda(AddressMode::XIndexedIndirect),
            0xA2        => self.ldx(AddressMode::Immediate),
            0xA3        => self.inv(), 
            0xA4        => self.ldy(AddressMode::Zeropage),
            0xA5        => self.lda(AddressMode::Zeropage),
            0xA6        => self.ldx(AddressMode::Zeropage),
            0xA7        => self.inv(),
            0xA8        => self.tay(),
            0xA9        => self.lda(AddressMode::Immediate),
            0xAA        => self.tax(),
            0xAB        => self.inv(),
            0xAC        => self.ldy(AddressMode::Absolute),
            0xAD        => self.lda(AddressMode::Absolute),
            0xAE        => self.ldx(AddressMode::Absolute),
            0xAF        => self.inv(),
            
            0xB0        => self.bcs(AddressMode::Relative),
            0xB1        => self.lda(AddressMode::IndirectYIndex),
            0xB2..=0xB3 => self.inv(), 
            0xB4        => self.ldy(AddressMode::ZeropageX),
            0xB5        => self.lda(AddressMode::ZeropageX),
            0xB6        => self.ldx(AddressMode::ZeropageY),
            0xB7        => self.inv(),
            0xB8        => self.clv(),
            0xB9        => self.lda(AddressMode::AbsoluteY),
            0xBA        => self.tsx(),
            0xBB        => self.inv(),
            0xBC        => self.ldy(AddressMode::AbsoluteX),
            0xBD        => self.lda(AddressMode::AbsoluteX),
            0xBE        => self.ldx(AddressMode::AbsoluteY),
            0xBF        => self.inv(),
            
            0xC0        => self.cpy(AddressMode::Immediate),
            0xC1        => self.cmp(AddressMode::XIndexedIndirect),
            0xC2..=0xC3 => self.inv(), 
            0xC4        => self.cpy(AddressMode::Zeropage),
            0xC5        => self.cmp(AddressMode::Zeropage),
            0xC6        => self.dec(AddressMode::Zeropage),
            0xC7        => self.inv(),
            0xC8        => self.iny(),
            0xC9        => self.cmp(AddressMode::Immediate),
            0xCA        => self.dex(),
            0xCB        => self.inv(),
            0xCC        => self.cpy(AddressMode::Absolute),
            0xCD        => self.cmp(AddressMode::Absolute),
            0xCE        => self.dec(AddressMode::Absolute),
            0xCF        => self.inv(),
            
            0xD0        => self.bne(AddressMode::Relative),
            0xD1        => self.cmp(AddressMode::IndirectYIndex),
            0xD2..=0xD4 => self.inv(), 
            0xD5        => self.cmp(AddressMode::ZeropageX),
            0xD6        => self.dec(AddressMode::ZeropageX),
            0xD7        => self.inv(),
            0xD8        => self.cld(),
            0xD9        => self.cmp(AddressMode::AbsoluteY),
            0xDA..=0xDC => self.inv(),
            0xDD        => self.cmp(AddressMode::AbsoluteX),
            0xDE        => self.dec(AddressMode::AbsoluteX),
            0xDF        => self.inv(),
            
            0xE0        => self.cpx(AddressMode::Immediate),
            0xE1        => self.sbc(AddressMode::XIndexedIndirect),
            0xE2..=0xE3 => self.inv(), 
            0xE4        => self.cpx(AddressMode::Zeropage),
            0xE5        => self.sbc(AddressMode::Zeropage),
            0xE6        => self.inc(AddressMode::Zeropage),
            0xE7        => self.inv(),
            0xE8        => self.inx(),
            0xE9        => self.sbc(AddressMode::Immediate),
            0xEA        => self.nop(),
            0xEB        => self.inv(),
            0xEC        => self.cpx(AddressMode::Absolute),
            0xED        => self.sbc(AddressMode::Absolute),
            0xEE        => self.inc(AddressMode::Absolute),
            0xEF        => self.inv(),
            
            0xF0        => self.beq(AddressMode::Relative),
            0xF1        => self.sbc(AddressMode::IndirectYIndex),
            0xF2..=0xF4 => self.inv(), 
            0xF5        => self.sbc(AddressMode::ZeropageX),
            0xF6        => self.inc(AddressMode::ZeropageX),
            0xF7        => self.inv(),
            0xF8        => self.sed(),
            0xF9        => self.sbc(AddressMode::AbsoluteY),
            0xFA..=0xFC => self.inv(),
            0xFD        => self.sbc(AddressMode::AbsoluteX),
            0xFE        => self.inc(AddressMode::AbsoluteX),
            0xFF        => self.inv()
        };
        cycles
    }

    fn get_operand(&mut self, cycles: &mut u8, mode: AddressMode) -> Operand {
        match mode {
            AddressMode::Accumulator => Operand {value: self.a.into(), address: 0},
            AddressMode::Absolute => {
                *cycles += 2;

                let address = self.read_memory_u16(self.pc);
                let value = self.read_memory_u8(address);
                self.pc += 2;
                Operand {value: value.into(), address: address}
            },
            AddressMode::AbsoluteX => {
                // TODO: carry?
                *cycles += 2;
                // Add 1 cycle if page break. 
                if ((self.pc % 0x100) + self.x as u16) > 0xFF {
                    *cycles += 1;
                }

                let address = self.read_memory_u16(self.pc) + self.x as u16;
                let value = self.read_memory_u8(address);
                self.pc += 2;
                Operand {value: value.into(), address}
            },
            AddressMode::AbsoluteY => { 
                // TODO: carry?
                *cycles += 2;
                // Add 1 cycle if page break. 
                if ((self.pc % 0x100) + self.y as u16) > 0xFF {
                    *cycles += 1;
                }

                let address = self.read_memory_u16(self.pc) + self.y as u16;
                let value = self.read_memory_u8(address);
                self.pc += 2;
                Operand {value: value.into(), address}
            },
            AddressMode::Immediate => {
                let value = self.read_memory_u8(self.pc);
                self.pc += 1;
                Operand {value: value.into(), address: 0}
            },
            AddressMode::Implied => Operand { value: 0, address: 0},
            AddressMode::Indirect => {
                *cycles += 3;

                let indir = self.read_memory_u16(self.pc);
                let address = self.read_memory_u16(indir);
                let value = self.read_memory_u8(address);
                self.pc += 2;
                Operand {value: value.into(), address}
            },
            AddressMode::XIndexedIndirect => {
                *cycles += 4;

                let indir = self.read_memory_u8(self.pc);
                let address = self.read_memory_u16(((indir as u16) + (self.x as u16))&0xFF);
                let value = self.read_memory_u8(address);
                self.pc += 1;
                Operand {value: value.into(), address}
            },
            AddressMode::IndirectYIndex => {
                *cycles += 3;
                if ((self.pc % 0x100) + self.y as u16) > 0xFF {
                    *cycles += 1;
                }

                let indir = self.read_memory_u8(self.pc);
                let address = self.read_memory_u16(indir as u16) + self.y as u16;
                let value = self.read_memory_u8(address);
                self.pc += 1;
                Operand {value: value.into(), address}
            },
            AddressMode::Relative => {
                // The value is actually i8, so we want to interpret it as such
                let value = self.read_memory_u8(self.pc) as i8;
                self.pc += 1;
                Operand {value: 0, address: self.pc.wrapping_add(value as u16)}
            },
            AddressMode::Zeropage => {
                *cycles += 1;

                let address = self.read_memory_u8(self.pc) as u16;
                self.pc += 1;
                Operand {value: self.read_memory_u8(address).into(), address}
            },
            AddressMode::ZeropageX => {
                *cycles += 2;

                let address = self.read_memory_u8(self.pc).wrapping_add(self.x) as u16;
                self.pc += 1;
                Operand {value: self.read_memory_u8(address).into(), address}
            },
            AddressMode::ZeropageY => {
                *cycles += 2;

                let address = self.read_memory_u8(self.pc).wrapping_add(self.y) as u16;
                self.pc += 1;
                Operand {value: self.read_memory_u8(address).into(), address}
            },
            AddressMode::Invalid => Operand {value: 0, address: 0}
        }
    }
    /// Add Memory to Accumulator with Carry
    fn adc(&mut self, mode: AddressMode) -> u8 {

        let mut cycles = 2;

        let operand_value = self.get_operand(&mut cycles, mode).value as u8;
        let result: u8;
        let carried;
        let carry = self.get_flag(FLAG_CARRY);
        if self.get_flag(FLAG_DECIMAL) {
            (result, carried) = self.a.overflowing_add_bcd(&operand_value, &carry);
        }
        else{
            let result_binary = (self.a as u16) + (operand_value as u16 + if carry {1} else {0});
            carried = result_binary >> 8 != 0;
            result = (result_binary & 0xFF) as u8;
        }

        // Reset all affectable flags
        self.set_flag(false, FLAG_CARRY | FLAG_OVERFLOW | FLAG_NEGATIVE | FLAG_ZERO);

        // If we got a value carry, set carry flag
        if carried {
            self.set_flag(true, FLAG_CARRY);
        }
        
        if result == 0 {
            self.set_flag(true, FLAG_ZERO);
        }
        
        // If value is signed, set negative flag
        if result & 0x80 != 0 {
            self.set_flag(true, FLAG_NEGATIVE);
        }
        
        // Set overflow flag if addition of two negative/positive numbers returned positive/negative number
        // See https://www.righto.com/2012/12/the-6502-overflow-flag-explained.html
        self.set_flag((!(operand_value ^ self.a) & (result ^ self.a)) & 0x80 != 0, FLAG_OVERFLOW);

        self.a = result;
        cycles
    }

    fn and(&mut self, mode: AddressMode) -> u8 {
        let mut cycles = 2;
        let operand_value = self.get_operand(&mut cycles, mode).value as u8;

        let result = self.a & operand_value;

        self.set_flag(result == 0, FLAG_ZERO);
        self.set_flag(result & 0x80 != 0, FLAG_NEGATIVE);

        self.a = result;
        cycles
    }

    // TODO: Cycle count is wrong here. REWORK CYCLE CALCULATION
    fn asl(&mut self, mode: AddressMode) -> u8 {
        let mut cycles = if mode == AddressMode::Accumulator {2} else {4};
        let operand = self.get_operand(&mut cycles, mode);

        // If 7 bit is 1, set carry flag
        self.set_flag(operand.value & 0x80 != 0, FLAG_CARRY);
        let result = (operand.value as u8) << 1;
        
        self.set_flag(result == 0, FLAG_ZERO);
        self.set_flag(result & 0x80 != 0, FLAG_NEGATIVE);

        if mode == AddressMode::Accumulator {
            self.a = result;
        }
        else{
            self.write_memory_u8(operand.address, result)
        }
        cycles
    }

    fn bcc(&mut self, mode: AddressMode) -> u8 {
        let mut cycles = 2;
        if !self.get_flag(FLAG_CARRY){
            let operand = self.get_operand(&mut cycles, mode);
            self.pc = operand.address;
        }
        else {
            self.pc += mode.address_size() as u16;
        }
        cycles
    }

    fn bcs(&mut self, mode: AddressMode) -> u8 {
        let mut cycles = 2;
        if self.get_flag(FLAG_CARRY){
            let operand = self.get_operand(&mut cycles, mode);
            self.pc = operand.address;
        }
        else {
            self.pc += mode.address_size() as u16;
        }
        cycles
    }

    fn beq(&mut self, mode: AddressMode) -> u8 {
        let mut cycles = 2;
        if self.get_flag(FLAG_ZERO){
            let operand = self.get_operand(&mut cycles, mode);
            self.pc = operand.address;
        }
        else {
            self.pc += mode.address_size() as u16;
        }
        cycles
    }

    fn bit(&mut self, mode: AddressMode) -> u8 {
        let mut cycles = 1;
        let operand = self.get_operand(&mut cycles, mode);
        let result = operand.value as u8 & self.a;
        self.set_flag(result == 0, FLAG_ZERO);
        self.set_flag(operand.value as u8 & FLAG_NEGATIVE != 0, FLAG_NEGATIVE);
        self.set_flag(operand.value as u8 & FLAG_OVERFLOW != 0, FLAG_OVERFLOW);
        cycles
    }

    fn bmi(&mut self, mode: AddressMode) -> u8 {
        let mut cycles = 2;
        if self.get_flag(FLAG_NEGATIVE){
            let operand = self.get_operand(&mut cycles, mode);
            self.pc = operand.address;
        }
        else {
            self.pc += mode.address_size() as u16;
        }
        cycles
    }

    fn bne(&mut self, mode: AddressMode) -> u8 {
        let mut cycles = 2;
        if !self.get_flag(FLAG_ZERO){
            let operand = self.get_operand(&mut cycles, mode);
            self.pc = operand.address;
        }
        else {
            self.pc += mode.address_size() as u16;
        }
        cycles
    }

    fn bpl(&mut self, mode: AddressMode) -> u8 {
        let mut cycles = 2;
        if !self.get_flag(FLAG_NEGATIVE){
            let operand = self.get_operand(&mut cycles, mode);
            self.pc = operand.address;
        }
        else {
            self.pc += mode.address_size() as u16;
        }
        cycles
    }

    fn brk(&mut self) -> u8 {
        self.write_memory_u16(stack_index!(self.sp), self.pc + 1);
        self.write_memory_u8(stack_index!(self.sp.wrapping_sub(2)), self.sr | FLAG_BREAK | FLAG_UNUSED);
        self.sp = self.sp.wrapping_sub(3);
        self.set_flag(true, FLAG_INTERRUPT);

        self.pc = self.read_memory_u16(IRQ_VECTOR);
        7
    }

    fn bvc(&mut self, mode: AddressMode) -> u8 {
        let mut cycles = 2;
        if !self.get_flag(FLAG_OVERFLOW){
            let operand = self.get_operand(&mut cycles, mode);
            self.pc = operand.address;
        }
        else {
            self.pc += mode.address_size() as u16;
        }
        cycles
    }

    fn bvs(&mut self, mode: AddressMode) -> u8 {
        let mut cycles = 2;
        if self.get_flag(FLAG_OVERFLOW){
            let operand = self.get_operand(&mut cycles, mode);
            self.pc = operand.address;
        }
        else {
            self.pc += mode.address_size() as u16;
        }
        cycles
    }

    fn clc(&mut self) -> u8 {
        self.set_flag(false, FLAG_CARRY);
        2
    }

    fn cld(&mut self) -> u8 {
        self.set_flag(false, FLAG_DECIMAL);
        2
    }

    fn cli(&mut self) -> u8 {
        self.set_flag(false, FLAG_INTERRUPT);
        2
    }

    fn clv(&mut self) -> u8 {
        self.set_flag(false, FLAG_OVERFLOW);
        2
    }

    fn cmp(&mut self, mode: AddressMode) -> u8 {
        let mut cycles = 1;
        let comparer = self.a;
        let operand = self.get_operand(&mut cycles, mode);
        let (result, _) = comparer.overflowing_sub(operand.value as u8);

        self.set_flag(result & 0x80 != 0, FLAG_NEGATIVE);
        self.set_flag(result == 0, FLAG_ZERO);
        self.set_flag(result == 0 || comparer > operand.value as u8, FLAG_CARRY);
        cycles
    }

    fn cpx(&mut self, mode: AddressMode) -> u8 {
        let mut cycles = 1;
        let comparer = self.x;
        let operand = self.get_operand(&mut cycles, mode);
        let (result, _) = comparer.overflowing_sub(operand.value as u8);

        self.set_flag(result & 0x80 != 0, FLAG_NEGATIVE);
        self.set_flag(result == 0, FLAG_ZERO);
        self.set_flag(result == 0 || comparer > operand.value as u8, FLAG_CARRY);
        cycles
    }

    fn cpy(&mut self, mode: AddressMode) -> u8 {
        let mut cycles = 1;
        let comparer = self.y;
        let operand = self.get_operand(&mut cycles, mode);
        let result = comparer.wrapping_sub(operand.value as u8);

        self.set_flag(result & 0x80 != 0, FLAG_NEGATIVE);
        self.set_flag(result == 0, FLAG_ZERO);
        self.set_flag(result == 0 || comparer > operand.value as u8, FLAG_CARRY);
        cycles
    }

    fn dec(&mut self, mode: AddressMode) -> u8 {
        let mut cycles = 1;
        let operand = self.get_operand(&mut cycles, mode);
        let result = (operand.value as u8).wrapping_sub(1);
        self.set_flag(result & 0x80 != 0, FLAG_NEGATIVE);
        self.set_flag(result == 0, FLAG_ZERO);
        self.write_memory_u8(operand.address, result);
        cycles
    }

    fn dex(&mut self) -> u8 {
        self.x = self.x.wrapping_sub(1);
        self.set_flag(self.x & 0x80 != 0, FLAG_NEGATIVE);
        self.set_flag(self.x == 0, FLAG_ZERO);
        2
    }
    
    fn dey(&mut self) -> u8 {
        self.y = self.y.wrapping_sub(1);
        self.set_flag(self.y & 0x80 != 0, FLAG_NEGATIVE);
        self.set_flag(self.y == 0, FLAG_ZERO);
        2
    }

    fn eor(&mut self, mode: AddressMode) -> u8 {
        let mut cycles = 2;
        let operand = self.get_operand(&mut cycles, mode);
        self.a ^= operand.value as u8;
        self.set_flag(self.a & 0x80 != 0, FLAG_NEGATIVE);
        self.set_flag(self.a == 0, FLAG_ZERO);
        cycles
    }

    fn inc(&mut self, mode: AddressMode) -> u8 {
        let mut cycles = 1;
        let operand = self.get_operand(&mut cycles, mode);
        let result = (operand.value as u8).wrapping_add(1);
        self.set_flag(result & 0x80 != 0, FLAG_NEGATIVE);
        self.set_flag(result == 0, FLAG_ZERO);
        self.write_memory_u8(operand.address, result);
        cycles
    }

    fn inx(&mut self) -> u8 {
        self.x = self.x.wrapping_add(1);
        self.set_flag(self.x & 0x80 != 0, FLAG_NEGATIVE);
        self.set_flag(self.x == 0, FLAG_ZERO);
        2
    }

    fn iny(&mut self) -> u8 {
        self.y = self.y.wrapping_add(1);
        self.set_flag(self.y & 0x80 != 0, FLAG_NEGATIVE);
        self.set_flag(self.y == 0, FLAG_ZERO);
        2
    }

    fn jmp(&mut self, mode: AddressMode) -> u8 {
        let mut cycles = 1;
        let operand = self.get_operand(&mut cycles, mode);
        self.pc = operand.address;
        cycles
    }

    fn jsr(&mut self, mode: AddressMode) -> u8 {
        let mut discard = 0;
        let operand = self.get_operand(&mut discard, mode);
        self.write_memory_u16(stack_index!(self.sp), self.pc - 1);
        self.pc = operand.address;
        self.sp = self.sp.wrapping_sub(2);
        6
    }

    fn lda(&mut self, mode: AddressMode) -> u8 {
        let mut cycles = 2;
        let operand = self.get_operand(&mut cycles, mode);
        self.a = operand.value as u8;
        self.set_flag(self.a & 0x80 != 0, FLAG_NEGATIVE);
        self.set_flag(self.a == 0, FLAG_ZERO);
        cycles
    }

    fn ldx(&mut self, mode: AddressMode) -> u8 {
        let mut cycles = 2;
        let operand = self.get_operand(&mut cycles, mode);
        self.x = operand.value as u8;
        self.set_flag(self.x & 0x80 != 0, FLAG_NEGATIVE);
        self.set_flag(self.x == 0, FLAG_ZERO);
        cycles
    }

    fn ldy(&mut self, mode: AddressMode) -> u8 {
        let mut cycles = 2;
        let operand = self.get_operand(&mut cycles, mode);
        self.y = operand.value as u8;
        self.set_flag(self.y & 0x80 != 0, FLAG_NEGATIVE);
        self.set_flag(self.y == 0, FLAG_ZERO);
        cycles
    }

    fn lsr(&mut self, mode: AddressMode) -> u8 {
        let mut cycles = if mode == AddressMode::Accumulator {2} else {4};
        let operand = self.get_operand(&mut cycles, mode);

        // If 0 bit is 1, set carry flag
        self.set_flag(operand.value & 0x01 != 0, FLAG_CARRY);
        let result = (operand.value as u8) >> 1;
        
        self.set_flag(result == 0, FLAG_ZERO);
        self.set_flag(false, FLAG_NEGATIVE);
        if mode == AddressMode::Accumulator {
            self.a = result;
        }
        else{
            self.write_memory_u8(operand.address, result)
        }
        cycles
    }

    fn nop(&mut self) -> u8 {
        2
    }

    fn ora(&mut self, mode: AddressMode) -> u8 {
        let mut cycles = 2;
        let operand_value = self.get_operand(&mut cycles, mode).value as u8;

        let result = self.a | operand_value;

        self.set_flag(result == 0, FLAG_ZERO);
        self.set_flag(result & 0x80 != 0, FLAG_NEGATIVE);

        self.a = result;

        cycles
    }

    fn pha(&mut self) -> u8 {
        self.write_memory_u8(stack_index!(self.sp), self.a);
        self.sp = self.sp.wrapping_sub(1);
        3
    }

    fn php(&mut self) -> u8 {
        self.write_memory_u8(stack_index!(self.sp), self.sr | FLAG_BREAK | FLAG_UNUSED);
        self.sp = self.sp.wrapping_sub(1);
        3
    }

    fn pla(&mut self) -> u8 {
        self.a = self.read_memory_u8(stack_index!(self.sp.wrapping_add(1)));
        
        self.set_flag(self.a & 0x80 != 0, FLAG_NEGATIVE);
        self.set_flag(self.a == 0, FLAG_ZERO);

        self.sp = self.sp.wrapping_add(1);
        4
    }

    fn plp(&mut self) -> u8 {
        self.sr = (self.read_memory_u8(stack_index!(self.sp.wrapping_add(1))) & !FLAG_BREAK) | FLAG_UNUSED;

        self.sp = self.sp.wrapping_add(1);
        4
    }

    fn rol(&mut self, mode: AddressMode) -> u8 {
        let mut cycles = if mode == AddressMode::Accumulator {2} else {4};
        let operand = self.get_operand(&mut cycles, mode);

        // If 0 bit is 1, set carry flag
        let result = (operand.value as u8) << 1 | if self.get_flag(FLAG_CARRY) {1} else {0};
        
        self.set_flag(operand.value & 0x80 != 0, FLAG_CARRY);
        self.set_flag(result == 0, FLAG_ZERO);
        self.set_flag(result & 0x80 != 0, FLAG_NEGATIVE);

        if mode == AddressMode::Accumulator {
            self.a = result;
        }
        else{
            self.write_memory_u8(operand.address, result)
        }
        cycles
    }

    fn ror(&mut self, mode: AddressMode) -> u8 {
        let mut cycles = if mode == AddressMode::Accumulator {2} else {4};
        let operand = self.get_operand(&mut cycles, mode);

        // If 0 bit is 1, set carry flag
        let result = (operand.value as u8) >> 1 | if self.get_flag(FLAG_CARRY) {0x80} else {0x00};
        
        self.set_flag(operand.value & 0x01 != 0, FLAG_CARRY);
        self.set_flag(result == 0, FLAG_ZERO);
        self.set_flag(result & 0x80 != 0, FLAG_NEGATIVE);

        if mode == AddressMode::Accumulator {
            self.a = result;
        }
        else{
            self.write_memory_u8(operand.address, result)
        }
        cycles
    }

    fn rti(&mut self) -> u8 {
        self.sr = (self.read_memory_u8(stack_index!(self.sp.wrapping_add(1))) & !FLAG_BREAK) | FLAG_UNUSED;

        self.pc = self.read_memory_u16(stack_index!(self.sp.wrapping_add(2)));

        self.sp = self.sp.wrapping_add(3);

        6
    }

    fn rts(&mut self) -> u8 {
        self.pc = self.read_memory_u16(stack_index!(self.sp.wrapping_add(1))) + 1;

        self.sp = self.sp.wrapping_add(2);
        6
    }

    fn sbc(&mut self, mode: AddressMode) -> u8 {
        let mut cycles = 2;

        let operand_value = self.get_operand(&mut cycles, mode).value as u8;
        let result: u8;
        let carried;
        let carry = self.get_flag(FLAG_CARRY);
        if self.get_flag(FLAG_DECIMAL) {
            (result, carried) = self.a.overflowing_sub_bcd(&operand_value, &carry);
        }
        else{
            let result_i32 = (self.a as i32 - operand_value as i32 - if carry {0} else {1}) as i32;
            result = (result_i32 & 0xFF) as u8;
            carried = result_i32 >> 8 != 0;
        }

        // carry flag is set via the complement of the carry status. E.g. set carry flag if carry did not happen
        self.set_flag(!carried, FLAG_CARRY);
        
        self.set_flag(result == 0, FLAG_ZERO);
        
        // If value is signed, set negative flag
        self.set_flag(result & 0x80 != 0, FLAG_NEGATIVE);

        self.set_flag(((operand_value ^ self.a) & (result ^ self.a)) & 0x80 != 0, FLAG_OVERFLOW);

        self.a = result;

        cycles
    }

    fn sec(&mut self) -> u8 {
        self.set_flag(true, FLAG_CARRY);
        2
    }
    
    fn sed(&mut self) -> u8 {
        self.set_flag(true, FLAG_DECIMAL);
        2
    }

    fn sei(&mut self) -> u8 {
        self.set_flag(true, FLAG_INTERRUPT);
        2
    }

    // TODO: this is likely a wrong cycle count
    fn sta(&mut self, mode: AddressMode) -> u8 {
        let mut cycles = 2;
        let operand = self.get_operand(&mut cycles, mode);
        self.write_memory_u8(operand.address, self.a);
        cycles
    }

    fn stx(&mut self, mode: AddressMode) -> u8 {
        let mut cycles = 2;
        let operand = self.get_operand(&mut cycles, mode);
        self.write_memory_u8(operand.address, self.x);
        cycles
    }

    fn sty(&mut self, mode: AddressMode) -> u8 {
        let mut cycles = 2;
        let operand = self.get_operand(&mut cycles, mode);
        self.write_memory_u8(operand.address, self.y);
        cycles
    }

    fn tax(&mut self) -> u8 {
        self.x = self.a;
        
        self.set_flag(self.x == 0, FLAG_ZERO);
        self.set_flag(self.x & 0x80 != 0, FLAG_NEGATIVE);

        2
    }

    fn tay(&mut self) -> u8 {
        self.y = self.a;
        
        self.set_flag(self.y == 0, FLAG_ZERO);
        self.set_flag(self.y & 0x80 != 0, FLAG_NEGATIVE);

        2
    }

    fn tsx(&mut self) -> u8 {
        self.x = self.sp;
        
        self.set_flag(self.x == 0, FLAG_ZERO);
        self.set_flag(self.x & 0x80 != 0, FLAG_NEGATIVE);

        2
    }

    fn txa(&mut self) -> u8 {
        self.a = self.x;
        
        self.set_flag(self.a == 0, FLAG_ZERO);
        self.set_flag(self.a & 0x80 != 0, FLAG_NEGATIVE);

        2
    }

    fn txs(&mut self) -> u8 {
        self.sp = self.x;
        2
    }

    fn tya(&mut self) -> u8 {
        self.a = self.y;
        
        self.set_flag(self.a == 0, FLAG_ZERO);
        self.set_flag(self.a & 0x80 != 0, FLAG_NEGATIVE);

        2
    }

    // Not actually a 6502 opcode, represent invalid opcodes
    fn inv(&mut self) -> u8 {
        panic!();
    }
}