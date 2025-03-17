use std::collections::HashMap;

const INSTRUCTION_SET: &[(&str, u16)] = &[
    ("HLT", 0b0000), ("ADD", 0b0001), ("SUB", 0b0010),
    ("NOR", 0b0011), ("AND", 0b0100), ("XOR", 0b0101),
    ("MOV", 0b0110), ("LDR", 0b0111), ("ADR", 0b1000),
    ("JMP", 0b1001), ("BRH", 0b1010), ("CAL", 0b1011),
    ("RET", 0b1100), ("PGE", 0b1101), ("LOD", 0b1110),
    ("STR", 0b1111)
];

const REGISTER_SET: &[(&str, u8)] = &[
    ("r0", 0b0000), ("r1", 0b0001), ("r2", 0b0010),
    ("r3", 0b0011), ("r4", 0b0100), ("r5", 0b0101),
    ("r6", 0b0110), ("r7", 0b0111), ("r8", 0b1000),
    ("r9", 0b1001), ("r10", 0b1010), ("r11", 0b1011),
    ("r12", 0b1100), ("r13", 0b1101), ("r14", 0b1110),
    ("r15", 0b1111)
];

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

        let instructions: HashMap<_, _> = INSTRUCTION_SET.iter().cloned().collect();
        let registers: HashMap<_, _> = REGISTER_SET.iter().cloned().collect();

        // first pass; collect keywords
        for parts in &parsed {
            if parts.len() == 3 && parts[0] == "define" {
                definitions.insert(
                    parts[1].clone(),
                    parts[2].parse::<u8>().unwrap()
                );
                address += 1; // i think?!
            } else if parts.len() == 1 && parts[0].ends_with(':') {
                labels.insert(
                    parts[0][..parts[0].len()-1].to_string(),
                    address
                );
            } else {
                address += 1;
            }
        }

        // second pass; assemble
        for parts in parsed {
            // skip keywords
            if parts.len() == 1 && parts[0].ends_with(':') { continue; }
            if parts[0] == "define" { continue; }

            let opcode = *instructions.get(parts[0].as_str()).expect("Unknown opcode");
            let mut instruction: u16 = opcode << 12;

            match parts[0].as_str() {
                "HLT" | "RET" => {
                    assert_eq!(parts.len(), 1, "Invalid operand count");
                    instruction |= 0b_0000_0000_0000_0000;
                },
                "LDR" | "ADR" => {
                    assert_eq!(parts.len(), 3, "Invalid operand count");
                    let reg_a = *registers.get(parts[1].as_str()).expect("Invalid register");
                    let value = match definitions.get(parts[2].as_str()) {
                        Some(def) => *def,
                        None => parts[2].parse::<u8>().expect("Invalid immediate value")
                    };
                    instruction |= (reg_a as u16) << 8;
                    instruction |= value as u16;
                }
                "JMP" | "CAL" | "BRH" => {
                    assert_eq!(parts.len(), 2, "Invalid operand count");
                    let address = labels.get(&parts[1]).copied()
                        .expect("Unknown label");
                    instruction |= address as u16;
                }
                _ => {
                    assert!(parts.len() >= 2, "Invalid operand count");
                    let reg_a = *registers.get(parts[1].as_str()).expect("Invalid register");
                    instruction |= (reg_a as u16) << 8;

                    if parts.len() > 2 {
                        let reg_b = *registers.get(parts[2].as_str()).expect("Invalid register");
                        instruction |= (reg_b as u16) << 4;
                    }

                    if parts.len() > 3 {
                        let reg_c = *registers.get(parts[3].as_str()).expect("Invalid register");
                        instruction |= reg_c as u16;
                    }
                }
            }

            assembled.push(instruction);
        }
        assembled
    }
}