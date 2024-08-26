import { Stack, Logger } from './helper';

type Instruction = {
	opcode: number,
	reg: {
		a: number,
		b: number,
		c: number
	},
	imm: number,
	addr: number,
	value: number
	condition: number,
}

class Processor {
	
	registers: Uint32Array;
	programData: Uint16Array;
	memory: Memory;
	counter: number;
	paused: boolean;
	addressStack: Stack<number>;
	debug: boolean;
	logger: Logger;

	constructor(registerCount: number, programData: Uint16Array, memory: Memory, logger: Logger) {
		
		this.registers = new Uint32Array(registerCount);
		this.programData = new Uint16Array(programData);
		this.counter = 0;
		this.memory = memory;
		this.addressStack = new Stack<number>();

		this.logger = logger; // this is *such* a stupid solution but, it works
		this.paused = false;
		this.debug = false;

	}
	
	decodeInstruction(instruction: number): Instruction {

		const opcode = (instruction >> 12) & 0b1111;  // Bits 15-12
		
		const regA = (instruction >> 8) & 0b1111;     // Bits 11-8
		const regB = (instruction >> 4) & 0b1111;  // Bits 7-4
		const regC = instruction & 0b1111;         // Bits 3-0
		
		const imm = instruction & 0b11111111; // Bits 7-0
		const addr = instruction & 0b1111111111; // Bits 9-0
		const value = instruction & 0b11111111; // Bits 7-0
		const cond = (instruction >> 10) & 0b11; // Bits 11-10
				
		return {
			opcode: opcode,
			reg: {a: regA, b: regB, c: regC},
			imm: imm,
			value: value,
			addr: addr,
			condition: cond
		};
		
	}										
	
	executeInstruction(instruction: Instruction) {
		switch (instruction.opcode) {
			case 0xf:
				// HLT - halt
				if (!this.paused) this.paused = true;
					this.counter = 0;	
					if (this.debug) this.logger.log("HLT - ", this.counter);
				break;
			case 0xe:
				// ADD - add
				this.registers[instruction.reg.c] = this.registers[instruction.reg.a] + this.registers[instruction.reg.b];
				this.counter += 1;
				if (this.debug) this.logger.log("ADD - ", this.registers[instruction.reg.c]);
				break;
			case 0xd:
				// SUB - subtract
				this.registers[instruction.reg.c] = this.registers[instruction.reg.a] - this.registers[instruction.reg.b];
				this.counter += 1;
				if (this.debug) this.logger.log("SUB - ", this.registers[instruction.reg.c]);
				break;
			case 0xc:
				// NOR - bitwise NOR
				this.registers[instruction.reg.c] = ~(this.registers[instruction.reg.a] | this.registers[instruction.reg.b]);
				this.counter += 1;
				if (this.debug) this.logger.log("NOR - ", this.registers[instruction.reg.c]);
				break;
			case 0xb:
				// AND - bitwise AND
				this.registers[instruction.reg.c] = this.registers[instruction.reg.a] & this.registers[instruction.reg.b];
				this.counter += 1;
				if (this.debug) this.logger.log("AND - ", this.registers[instruction.reg.c]);
				break;
			case 0xa:
				// XOR - bitwise XOR    		
				this.registers[instruction.reg.c] = this.registers[instruction.reg.a] ^ this.registers[instruction.reg.b];
				this.counter += 1;
				if (this.debug) this.logger.log("XOR - ", this.registers[instruction.reg.c]);
				break;
			case 0x9:
				// MOV - move		
				this.registers[instruction.reg.c] = this.registers[instruction.reg.a];
				this.counter += 1;
				if (this.debug) this.logger.log("MOV - ", this.registers[instruction.reg.c]);
				break;
			case 0x8:
				// LDR - load register
				this.registers[instruction.reg.a] = instruction.value;
				this.counter += 1;
				if (this.debug) this.logger.log("LDR - ", this.registers[instruction.reg.a]);
				break;
			case 0x7:
				// ADR = add register
				this.registers[instruction.reg.a] += instruction.value;
				this.counter += 1;
				if (this.debug) this.logger.log("ADR - ", this.registers[instruction.reg.a]);
				break;
			case 0x6:			
				// JMP = jump
				this.counter = instruction.addr;
				if (this.debug) this.logger.log("JMP - ", instruction.addr);
				break;
			case 0x5:
				// BRH = branch
				switch (instruction.condition) {
					case 0b00:
						// BEQ = branch if equal
						if (this.registers[instruction.reg.a] === this.registers[instruction.reg.b]) {
							this.counter = instruction.addr;
							if (this.debug) this.logger.log("BRH/BEQ - ", instruction.addr);
						}
						break;
					case 0b01:
						// BNE = branch if not equal
						if (this.registers[instruction.reg.a] !== this.registers[instruction.reg.b]) {
							this.counter = instruction.addr;
							if (this.debug) this.logger.log("BRH/BNE - ", instruction.addr);
						}
						break;
					case 0b10:
						// BLT = branch if less than
						if (this.registers[instruction.reg.a] < this.registers[instruction.reg.b]) {
							this.counter = instruction.addr;
							if (this.debug) this.logger.log("BRH/BLT - ", instruction.addr);
						}
						break;
					case 0b11:
						// BGT = branch if greater than
						if (this.registers[instruction.reg.a] > this.registers[instruction.reg.b]) {
							this.counter = instruction.addr;
							if (this.debug) this.logger.log("BRH/BGT - ", instruction.addr);
						}
						break;
					default: 
						if (this.debug) this.logger.log("BRH/NOP - No condition provided");
						this.counter += 1;
				}
				break;
			case 0x4:
				// CAL = call
				this.counter = instruction.addr;
				this.addressStack.push(this.counter + 1);
				if (this.debug) this.logger.log("CAL - ", instruction.addr);
				break;
			case 0x3:
				// RET = return
				this.counter = this.addressStack.items[0]
				this.addressStack.pop();
				if (this.debug) this.logger.log("RET - ", this.addressStack.items[0]);
				break;
			case 0x2:
				// PGE = page memory
				this.memory.setPage(instruction.reg.a);
			case 0x1:
				// LOD = load from memory                                                    // dirty fix
				this.registers[instruction.reg.b] = this.memory.read((instruction.reg.a + instruction.reg.c));
				this.counter += 1;
				if (this.debug) this.logger.log("LOD - ", this.registers[instruction.reg.b]);
				break;			
			case 0x0:
				// STR = store to memory
				this.memory.write((instruction.reg.a + instruction.reg.c), this.registers[instruction.reg.b]);			
				this.counter += 1;
				if (this.debug) this.logger.log("STR - ", this.memory.read((instruction.reg.a + instruction.reg.c)));
				break;
			default:
				console.error("Invalid opcode, resetting program");			
				this.paused = true;
				this.counter = 0;
		}
	}

	executeProgram() {
		this.counter = 0
		for (let i = 0; i < this.programData.length; i++) {
			let instruction = this.decodeInstruction(this.programData[this.counter]);
			if(this.debug) this.logger.log("instruction: ", Object.entries(instruction), "\nraw: ", this.programData[this.counter], "\ncounter: ", this.counter);
			
			this.executeInstruction(instruction);
		}
	}
}

class Memory {
	private offset = 0;
	memory: Uint8Array;

	constructor(size: number) {
		this.memory = new Uint8Array(size);
	}
	
	read(address: number): any {
		return this.memory[address + this.offset];
	}
	
	write(address: number, value: number): void {
		this.memory[address + this.offset] = value;
	}		
	
	setPage(page: number): void {
		this.offset = page * 256; 
	}		

}

export class putater {

	cpu: Processor
	memory: Memory
	logger: Logger

	constructor() {
		this.memory = new Memory(4096);
		this.logger = new Logger();
		this.cpu = new Processor(16, new Uint16Array(16), this.memory, this.logger);		
	}							

	loadProgram(program: Uint16Array) {
		this.cpu.programData = program;
	}

	runProgram() {
		this.cpu.logger.log("Program execution started");
		this.cpu.executeProgram();
		this.cpu.logger.log("Program execution complete");
	}

}								