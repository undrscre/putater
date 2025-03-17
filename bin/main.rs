use putater::cpu;
use putater::assembler;

use std::fs;
use std::env;

fn main() {
    env_logger::builder().filter_level(log::LevelFilter::Debug).init();

    let args: Vec<String> = env::args().collect();
    let contents = fs::read_to_string(&args[1])
        .expect("Should have been able to read the file");

    let assembled = assembler::Assembler::assemble(contents);

    let mut cpu = cpu::CPU::new(assembled, [0; 16]);
    cpu.run();
}