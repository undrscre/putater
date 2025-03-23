use std::io::{Read, Seek, SeekFrom, Write};

use crate::memory::MemoryBus;
use bitflags::bitflags;
use log::{debug, info, warn};

// 16-bit registers rewrite o_0

bitflags! {
    #[derive(Debug)]
    pub struct Flags: u8 {
        const ZERO = 0b0000_0001;     
        const CARRY = 0b0000_0010;    
        const NEGATIVE = 0b0000_0100; 
        const OVERFLOW = 0b0000_1000; 
    }
}

pub struct CPU {
    pub program: Vec<u16>,
    pub memory: MemoryBus,
    pub registers: [u16; 16],

    pub pc: usize,
    pub address_stack: Vec<usize>,
    pub flags: Flags,
}

impl CPU {
    pub fn new(program: Vec<u16>, registers: [u16; 16]) -> CPU {
        CPU {
            registers,
            memory: MemoryBus::new(),
            program,

            pc: 0,
            address_stack: vec![],
            flags: Flags::empty(),
        }
    }

    fn set_flags(&mut self, result: u16, reg_a: usize, reg_b: usize, reg_c: usize, carry: bool) {
        self.flags.set(Flags::ZERO, result == 0);
        self.flags.set(Flags::CARRY, carry);
        self.flags.set(Flags::NEGATIVE, (result & 0x8000) != 0);

        let a = self.registers[reg_a] as i16;
        let b = self.registers[reg_b] as i16;
        let c = self.registers[reg_c] as i16;
        self.flags.set(
            Flags::OVERFLOW,
            (a > 0 && b > 0 && c < 0) || (a < 0 && b < 0 && c > 0),
        );
    }

    fn split_u16(&self, value: u16) -> (u8, u8) {
        let low_byte = (value & 0xFF) as u8;
        let high_byte = ((value >> 8) & 0xFF) as u8;
        (low_byte, high_byte)
    }

    pub fn run(&mut self) {
        info!("running program!");
        loop {
            let instr = self.program[self.pc] as u16;
            let opcode = (instr >> 12) as u8;

            let reg_a = ((instr >> 8) & 0b1111) as usize;
            let reg_b = ((instr >> 4) & 0b1111) as usize;
            let reg_c = (instr & 0b1111) as usize;

            let value = (instr & 0xFF) as u8;

            debug!("stepping.. current state: {:?} {:?}", self.registers, self.flags);
            self.pc += 1;
            match opcode {
                0b0000 => {
                    info!("Program called HLT, stopping execution..");
                    break;
                }
                0b0001 => {
                    let (result, carry) = self.registers[reg_a].overflowing_add(self.registers[reg_b]);
                    self.registers[reg_c] = result;
                    self.set_flags(result, reg_a, reg_b, reg_c, carry);
                    debug!("executed. ADD: {0}", self.registers[reg_c]);
                }
                0b0010 => {
                    let (result, carry) = self.registers[reg_a].overflowing_sub(self.registers[reg_b]);
                    self.registers[reg_c] = result;
                    self.set_flags(result, reg_a, reg_b, reg_c, carry);
                    debug!("executed. SUB: {0}", self.registers[reg_c]);
                }
                0b0011 => {
                    let result = !(self.registers[reg_a] | self.registers[reg_b]);
                    self.registers[reg_c] = result;
                    self.set_flags(result, reg_a, reg_b, reg_c, false);
                    debug!("executed. NOR: {0}", self.registers[reg_c]);
                }
                0b0100 => {
                    let result = self.registers[reg_a] & self.registers[reg_b];
                    self.registers[reg_c] = result;
                    self.set_flags(result, reg_a, reg_b, reg_c, false);
                    debug!("executed. AND: {0}", self.registers[reg_c]);
                }
                0b0101 => {
                    let result = self.registers[reg_a] ^ self.registers[reg_b];
                    self.registers[reg_c] = result;
                    self.set_flags(result, reg_a, reg_b, reg_c, false);
                    debug!("executed. XOR: {0}", self.registers[reg_c]);
                }
                0b0110 => {
                    self.registers[reg_b] = self.registers[reg_a];
                    debug!("executed. MOV: {0}", self.registers[reg_c]);
                }
                0b0111 => {
                    self.registers[reg_a] = value as u16;
                    debug!("executed. LDR: {0}", self.registers[reg_a]);
                }
                0b1000 => {
                    let (result, carry) = self.registers[reg_a].overflowing_add(value as u16);
                    self.registers[reg_a] = result;
                    // i think this is how you do it
                    self.set_flags(result, reg_a, reg_b, reg_b, carry);
                    debug!("executed. ADR: {0}", self.registers[reg_a]);
                }
                0b1001 => {
                    self.pc = value as usize;
                    debug!("executed. JMP: {0}", self.pc);
                }
                0b1010 => {
                    let should_branch = match reg_a {
                        0b0000 => !self.flags.contains(Flags::ZERO), 
                        0b0001 => self.flags.contains(Flags::ZERO),  
                        0b0010 => !self.flags.contains(Flags::NEGATIVE) && !self.flags.contains(Flags::ZERO),
                        0b0011 => self.flags.contains(Flags::NEGATIVE), 
                        0b0100 => !self.flags.contains(Flags::CARRY),
                        0b0101 => self.flags.contains(Flags::CARRY),
                        0b0110 => !self.flags.contains(Flags::OVERFLOW),
                        0b0111 => self.flags.contains(Flags::OVERFLOW),
                        _ => false,
                    };

                    if should_branch {
                        self.address_stack.push(self.pc);
                        self.pc = value as usize;
                    }

                    debug!("executed. BRH: {0}", should_branch);
                }
                0b1011 => {
                    self.address_stack.push(self.pc);
                    self.pc = value as usize;
                    debug!("executed. CAL: {0}", self.pc);
                }
                0b1100 => {
                    self.pc = self.address_stack.pop().unwrap_or(0); 
                    debug!("executed. RET: {0}", self.pc);
                }
                0b1101 => {
                    self.memory.page = value;
                    debug!("executed. PGE: {0}", self.memory.page);
                }
                0b1110 => {
                    let _ = self.memory.seek(SeekFrom::Start(value as u64));
                    let buf = &mut [0u8; 2];
                    self.memory.read_exact(buf).expect("read to memory should succeed");
                    self.registers[reg_a] = u16::from_le_bytes(*buf);
                    debug!("executed. LOD: {0}", self.registers[reg_a]);
                }
                0b1111 => {
                    let _ = self.memory.seek(SeekFrom::Start(value as u64));
                    let (low, high) = self.split_u16(self.registers[reg_a]);
                    let mut buf;

                    if self.registers[reg_a] <= 255 {
                        // added this because i'm too lazy to
                        // rewrite the device spec
                        buf = [self.registers[reg_a] as u8, value];
                    } else {
                        buf = [low, high];
                    }
                    
                    self.memory
                        .write(&mut buf)
                        .expect("write to memory should succeed");
                    debug!("executed. STR: {:?}", buf);
                }
                _ => unreachable!(),
            }
        }
    }
}
