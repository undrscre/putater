import { Stack } from './helper';

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
	paused: Boolean;
	addressStack: Stack<number>;

	constructor(registerCount: number, programData: Uint16Array, memory: Memory) {
		
		this.registers = new Uint32Array(registerCount);
		this.programData = new Uint16Array(programData);
		this.counter = 0;
		this.memory = memory;
		this.addressStack = new Stack<number>();

		this.paused = false;
		
	}
	
	decodeInstruction(instruction: number): Instruction {
		
		let opcode = (instruction >> 12) & 0b1111;  // Bits 15-12
		opcode = parseInt(opcode.toString(16), 16);
		
		const regA = (instruction >> 8) & 0b1111;     // Bits 11-8
		const regB = (instruction >> 4) & 0b1111;  // Bits 7-4
		const regC = instruction & 0b1111;         // Bits 3-0
		
		const imm = instruction & 0b11111111;
		const addr = instruction & 0b1111111111;
		const value = instruction & 0b11111111; // Bits 7-0
		const cond = ((instruction & 0b110000000000) >> 10);
		
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
			case 0x0				:
				// NOP - no operation
				this.counter += 1;
				break;
			case 0x1:
				// HLT - halt
				if (!this.paused) this.paused = true;
				this.counter = 0;	
				break;
			case 0x2:
				// ADD - add
				this.registers[instruction.reg.c] = this.registers[instruction.reg.a] + this.registers[instruction.reg.b];
				this.counter += 1;
				break;
			case 0x3:
				// SUB - subtract
				this.registers[instruction.reg.c] = this.registers[instruction.reg.a] - this.registers[instruction.reg.b];
				this.counter += 1;
				break;
			case 0x4:
				// NOR - bitwise NOR
				this.registers[instruction.reg.c] = ~(this.registers[instruction.reg.a] | this.registers[instruction.reg.b]);
				this.counter += 1;
				break;
			case 0x5:
				// AND - bitwise AND
				this.registers[instruction.reg.c] = this.registers[instruction.reg.a] & this.registers[instruction.reg.b];
				this.counter += 1;
				break;
			case 0x6:
				// XOR - bitwise XOR    
				this.registers[instruction.reg.c] = this.registers[instruction.reg.a] ^ this.registers[instruction.reg.b];
				this.counter += 1;
				break;
			case 0x7:
				// MOV - move
				this.registers[instruction.reg.c] = this.registers[instruction.reg.a];
				this.counter += 1;
				break;
			case 0x8:
				// LDR - load register
				this.registers[instruction.reg.a] = instruction.value;
				this.counter += 1;
				break;
			case 0x9:
				// ADR = add register
				this.registers[instruction.reg.a] += instruction.value;
				this.counter += 1;
				break;
			case 0xA:			
				// JMP = jump
				this.counter = instruction.addr;
				break;
			case 0xB:
				// BRH = branch
				switch (instruction.condition) {
					case 0b00:
						// BEQ = branch if equal
						if (this.registers[instruction.reg.a] === this.registers[instruction.reg.b]) {
							this.counter = instruction.addr;
						}
						break;
					case 0b01:
						// BNE = branch if not equal
						if (this.registers[instruction.reg.a] !== this.registers[instruction.reg.b]) {
							this.counter = instruction.addr;
						}
						break;
					case 0b10:
						// BLT = branch if less than
						if (this.registers[instruction.reg.a] < this.registers[instruction.reg.b]) {
							this.counter = instruction.addr;
						}
						break;
					case 0b11:
						// BGT = branch if greater than
						if (this.registers[instruction.reg.a] > this.registers[instruction.reg.b]) {
							this.counter = instruction.addr;
						}
						break;
					default: 
						this.counter += 1;
				}
				break;
			case 0xC:
				// CAL = call
				this.counter = instruction.addr;
				this.addressStack.push(this.counter + 1);
				break;
			case 0xD:
				// RET = return
				this.counter = this.addressStack.items[0]
				this.addressStack.pop();
				break;
			case 0xE:
				// LOD = load from memory                                                    // dirty fix
				this.registers[instruction.reg.b] = this.memory.read((instruction.reg.a + instruction.reg.c));
				break;			
			case 0xF:
				// STR = store to memory
				this.memory.write((instruction.reg.a + instruction.reg.c), this.registers[instruction.reg.b]);
				break;
		}
	}

	executeProgram() {
		while (!this.paused) {
			let instruction = this.decodeInstruction(this.programData[this.counter]);
			this.executeInstruction(instruction);
		}
	}
}

class Memory {
	memory: Uint8Array;
	constructor(size: number) {
		this.memory = new Uint8Array(size * 1024);
	}
	
	read(address: number): any {
		return this.memory[address];
	}
	
	write(address: number, value: number): void {
		this.memory[address] = value;
	}		
	
}

export class putater {
	cpu: Processor
	memory: Memory
	
	constructor() {
		this.memory = new Memory(16);
		this.cpu = new Processor(16, new Uint16Array(16), this.memory);		

		this.cpu.programData = new Uint16Array([
			0b0111000010000000,
			0b0111000110000000,
			0b0111001000000000,
			0b1101				
		]);
		this.cpu.executeProgram();
	}
}