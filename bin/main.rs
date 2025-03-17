use vm::cpu;

fn main() {
    env_logger::builder().filter_level(log::LevelFilter::Debug).init();

    let registers = [0; 16];

    let program = vec![
        0b0111_0000_0000_0001, // prepare registers
        0b0111_0001_0000_0000,

        0b1101_0000_1111_1010, // set page 250
        0b1111_0000_0001_0000, // write

        0b1101_0000_1111_1011, // set page 251
        0b1110_0000_0001_0000, // write

        0b1101_0000_1111_1100, // set page 252
        0b1111_0000_0001_0000, // write (will warn i think)

        0b0000_0000_0000_0000  // HLT
    ];

    let mut cpu = cpu::CPU::new(program, registers);
    cpu.run();
}