use vm::cpu;

fn main() {
    env_logger::builder().filter_level(log::LevelFilter::Debug).init();

    let mut registers = [0; 16];
    registers[0] = 1;
    registers[1] = 1;

    let program = vec![
        0b0001_0001_0000_0010,  // ADD R1, R0, R2
        0b0001_0010_0001_0011,  // ADD R2, R1, R3
        0b0001_0011_0010_0100,  // ADD R3, R2, R4
        0b0001_0100_0011_0101,  // ADD R4, R3, R5
        0b0001_0101_0100_0110,  // ADD R5, R4, R6
        0b0001_0110_0101_0111,  // ADD R6, R5, R7
        
        // Calculate checksum in R15 (XOR all values)
        0b0101_0000_0001_1111,  // XOR R0, R1, R15
        0b0101_1111_0010_1111,  // XOR R15, R2, R15
        0b0101_1111_0011_1111,  // XOR R15, R3, R15
        0b0101_1111_0100_1111,  // XOR R15, R4, R15
        0b0101_1111_0101_1111,  // XOR R15, R5, R15
        0b0101_1111_0110_1111,  // XOR R15, R6, R15
        0b0101_1111_0111_1111,  // XOR R15, R7, R15
        
        0b0000_0000_0000_0000   // HLT
    ];

    let mut cpu = cpu::CPU::new(program, registers);
    cpu.run();
}