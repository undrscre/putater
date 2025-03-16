use bitflags::bitflags;
use log::{debug, info, warn};
use crate::memory::MemoryBus;

bitflags! {
    pub struct Flags: u8 {
        const Z = 0b0001;
        const C = 0b0010;
        const N = 0b0100;
        const V = 0b1000;
   }
}

pub struct CPU {
    pub program: Vec<u16>,
    pub memory: MemoryBus,
    pub registers: [u8; 16],

    pub page: u8,
    pub pc: usize,
    _flags: Flags
}

impl CPU {
    pub fn new(program: Vec<u16>, registers: [u8; 16]) -> CPU {
        CPU {
            registers,
            memory: MemoryBus::new(),
            program,

            page: 0,
            pc: 0,
            _flags: Flags::empty()
        }
    }

    pub fn run(&mut self) {
        // genuinely should find a better way to impl this
        // used for cases like loops so it doesn't break mid program
        // TODO: mut later
        info!("running program!");
        let lifeline = 0;
        loop {

            let instr = self.program[self.pc] as u16;
            let opcode = (instr >> 12) as u8;

            let reg_a = ((instr >> 8) & 0b1111) as usize;
            let reg_b = ((instr >> 4) & 0b1111) as usize;
            let reg_c = (instr & 0b1111) as usize;
            
            let value = (instr & 0xFF) as u8;

            debug!("stepping.. current state: {:?}", self.registers);
            self.pc += 1;

            if self.pc > (4096 + lifeline) {
                warn!("PC reached execution limit");
                break
            }

            match opcode {
                0b0000 => {
                    info!("Program called HLT, stopping execution..");
                    break;
                },
                0b0001 => {
                    self.registers[reg_c] = self.registers[reg_a] + self.registers[reg_b];
                    debug!("exec. ADD: {0}", self.registers[reg_c]);
                },
                0b0010 => {
                    self.registers[reg_c] = self.registers[reg_a] - self.registers[reg_b];
                    debug!("exec. SUB: {0}", self.registers[reg_c]);
                },
                0b0011 => {
                    self.registers[reg_c] = !(self.registers[reg_a] | self.registers[reg_b]);
                    debug!("exec. NOR: {0}", self.registers[reg_c]);
                },
                0b0100 => {
                    self.registers[reg_c] = self.registers[reg_a] & self.registers[reg_b];
                    debug!("exec. AND: {0}", self.registers[reg_c]);
                },
                0b0101 => {
                    self.registers[reg_c] = self.registers[reg_a] ^ self.registers[reg_b];
                    debug!("exec. XOR: {0}", self.registers[reg_c]);
                },
                0b0110 => {
                    self.registers[reg_c] = self.registers[reg_a];
                    debug!("exec. MOV: {0}", self.registers[reg_c]);
                },
                0b0111 => {
                    self.registers[reg_a] = value;
                    debug!("exec. LDR: {0}", self.registers[reg_a]);
                }
                0b1000 => {
                    self.registers[reg_a] += value;
                    debug!("exec. LDR: {0}", self.registers[reg_a]);
                },
                0b1001 => {
                    //JMP
                    todo!()
                },
                0b1010 => {
                    //BRH
                    todo!()
                },
                0b1011 => {
                    //CAL
                    todo!()
                },
                0b1100 => {
                    //RET
                    todo!()
                },
                0b1101 => {
                    self.page = value;
                    debug!("exec. PGE: {0}", self.page);
                },
                0b1110 => {
                    self.registers[reg_a] = self.memory.read(self.page, self.registers[reg_b]);
                    debug!("exec. LOD: {0}", self.registers[reg_a]);
                },
                0b1111 => {
                    self.memory.write(self.page, self.registers[reg_b], self.registers[reg_a]);
                    debug!("exec. STR: {0}", self.memory.read(self.page, self.registers[reg_b]));
                },
                _ => {
                    // shouldn't be possible like ever if you see this run
                    warn!("Unknown opcode {opcode}, halting..");
                    break;
                }
            }

        }
    }
}