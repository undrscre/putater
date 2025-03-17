use bitflags::bitflags;
use std::collections::HashMap;

bitflags! {
    struct InstructionSet: u8 {
        const HLT = 0b0000;
        const ADD = 0b0001;
        const SUB = 0b0010;

        const NOR = 0b0011;
        const AND = 0b0100;
        const XOR = 0b0101;

        const MOV = 0b0110;
        const LDR = 0b0111;
        const ADR = 0b1000;

        const JMP = 0b1001;
        const BRH = 0b1010;
        const CAL = 0b1011;

        const RET = 0b1100;
        const PGE = 0b1101;
        const LOD = 0b1110;

        const STR = 0b1111;
    }

    struct RegisterSet: u8 {
        const R0 = 0b0000;
        const R1 = 0b0001;
        const R2 = 0b0010;
        const R3 = 0b0011;
        const R4 = 0b0100;
        const R5 = 0b0101;
        const R6 = 0b0110;
        const R7 = 0b0111;
        const R8 = 0b1000;
        const R9 = 0b1001;
        const R10 = 0b1010;
        const R11 = 0b1011;
        const R12 = 0b1100;
        const R13 = 0b1101;
        const R14 = 0b1110;
        const R15 = 0b1111;
    }
}

pub struct Assembler;
impl Assembler {
    fn parse(code: String) -> Vec<Vec<String>> {
        code.lines()
            .filter(|l| !l.trim().is_empty() && !l.trim().starts_with(';'))
            .map(|l| l.trim().split_whitespace().map(String::from).collect())
            .collect()
    }

    pub fn assemble(code: String) -> Vec<u16> {
        let mut assembled: Vec<u16> = Vec::new();
        let mut definitions: HashMap<String, u8> = HashMap::new();
        let mut labels: HashMap<String, u8> = HashMap::new();
        let mut address: u8 = 0;

        let parsed = Self::parse(code);

        // first pass; collect keywords
        for parts in &parsed {
            if parts.len() == 3 && parts[0] == "define" {
                definitions.insert(parts[1].clone(), parts[2].parse::<u8>().unwrap());
                address += 1; // i think?!
            } else if parts.len() == 1 && parts[0].ends_with(':') {
                labels.insert(parts[0][..parts[0].len() - 1].to_string(), address);
            } else {
                address += 1;
            }
        }

        // second pass; assemble
        for parts in parsed {
            // skip keywords
            if parts.len() == 1 && parts[0].ends_with(':') {
                continue;
            }
            if parts[0] == "define" {
                continue;
            }

            let opcode = InstructionSet::from_name(parts[0].as_str()).expect("opcode should exist");
            let mut instruction = (opcode.bits() as u16) << 12;

            let get_register = |part: &String| RegisterSet::from_name(part.to_uppercase().as_str()).expect("register should exist").bits();
            match parts[0].as_str() {
                "HLT" | "RET" => {
                    assert_eq!(parts.len(), 1, "Invalid operand count");
                    instruction |= 0b_0000_0000_0000_0000;
                }
                "LDR" | "ADR" => {
                    assert_eq!(parts.len(), 3, "Invalid operand count");
                    let reg_a = get_register(&parts[1]);
                    let value = match definitions.get(parts[2].as_str()) {
                        Some(def) => *def,
                        None => parts[2].parse::<u8>().expect("Invalid immediate value"),
                    };
                    instruction |= (reg_a as u16) << 8;
                    instruction |= value as u16;
                }
                "JMP" | "CAL" | "BRH" => {
                    assert_eq!(parts.len(), 2, "Invalid operand count");
                    let address = labels.get(&parts[1]).copied().expect("Unknown label");
                    instruction |= address as u16;
                }
                _ => {
                    assert!(parts.len() >= 2, "Invalid operand count");
                    let reg_a = get_register(&parts[1]);
                    instruction |= (reg_a as u16) << 8;

                    if parts.len() > 2 {
                        let reg_b = get_register(&parts[2]);
                        instruction |= (reg_b as u16) << 4;
                    }

                    if parts.len() > 3 {
                        let reg_c = get_register(&parts[3]);
                        instruction |= reg_c as u16;
                    }
                }
            }

            assembled.push(instruction);
        }
        assembled
    }
}
