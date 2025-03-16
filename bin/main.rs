use vm::cpu;

fn main() {
    env_logger::builder().filter_level(log::LevelFilter::Debug).init();

    let registers = [0; 16];

    let program = vec![
        0b0111_0000_0000_0001,
        0b0111_0001_0000_0000,
        0b1101_0000_1111_1010,
        0b1111_0000_0001_0000,
        0b0000_0000_0000_0000   // HLT
    ];

    let mut cpu = cpu::CPU::new(program, registers);
    cpu.run();
}