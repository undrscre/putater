use bitflags::bitflags;
use log::{debug, info, warn};

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

    pub registers: [u8; 16],
    _ram: [u8; 65535],
    pub pc: usize,
    _flags: Flags
}

impl CPU {
    pub fn new(program: Vec<u16>, registers: [u8; 16]) -> CPU {
        CPU {
            registers,
            _ram: [0; 65535],
            program,

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
                }
                _ => {
                    // shouldn't be possible like ever if you see this run
                    warn!("Unknown opcode {opcode}, halting..");
                    break;
                }
            }

        }
    }
}