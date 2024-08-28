import { dec2bin } from "./helper";

const instructionSet: { [code: string]: number } = {
    "HLT": 0b1111,
    "ADD": 0b1110,
    "SUB": 0b1101,
    "NOR": 0b1100,
    "AND": 0b1011,
    "XOR": 0b1010,
    "MOV": 0b1001,
    "LDR": 0b1000,
    "ADR": 0b0111,
    "JMP": 0b0110,
    "BRH": 0b0101,
    "CAL": 0b0100,
    "RET": 0b0011,
    "PGE": 0b0010,
    "LOD": 0b0001,
    "STR": 0b0000
}

const registerSet: { [code: string]: number } = {
    "r0":  0b0000,
    "r1":  0b0001,
    "r2":  0b0010,
    "r3":  0b0011,
    "r4":  0b0100,
    "r5":  0b0101,
    "r6":  0b0110,
    "r7":  0b0111,
    "r8":  0b1000,
    "r9":  0b1001,
    "r10": 0b1010,
    "r11": 0b1011,
    "r12": 0b1100,
    "r13": 0b1101,
    "r14": 0b1110,
    "r15": 0b1111
}

export class Assembler {
    parse(code: string): string[][] {
        const lines = code.split("\n");
        const parsed: string[][] = [];

        for (const line of lines) {
            const trimmed = line.trim();
            if (trimmed.length === 0 || trimmed.startsWith(";")) {
                continue
            };
            const parts = trimmed.split(" ");
            parsed.push(parts);
        };

        return parsed
    }

    assemble(parsed: string[][]): Uint16Array {
        let assembled: Uint16Array = new Uint16Array([]);
        let labels: { [label: string]: number } = {};
        let defintions: { [label: string]: number } = {};
        let instructions: number[][] = [];
        let address = 0;

        // pass one, translate defintions
        for (const parts of parsed) {
            if (parts.length === 3 && parts[0] === "define") {
                defintions[parts[1]] = parseInt(parts[2]);
            }
        }

        // pass two, translate labels
        for (const parts of parsed) {
            if (parts.length === 1 && parts[0].endsWith(":")) {
                const label = parts[0].slice(0,-1);
                labels[label] = address;
            } else {
                address++;
            }
        }

        // pass four, translate instructions
        for (let pc = 0; pc < parsed.length; pc++) {
            const parts = parsed[pc];
            let instruction: number[] = [];

            if (parts.length === 1 && parts[0].endsWith(":")) {
                continue;
            }
            if (parts[0] === "define") {
                continue;
            }

            const opcode = instructionSet[parts[0]];

            if (opcode === undefined) {
                throw new Error(`Unknown opcode: ${parts[0]}`);
            } else {
                instruction.push(opcode);
            }

            // check for correct operands length
            if (["HLT","RET"].includes(parts[0]) && parts.length !== 1) {
                throw new Error(`Invalid number of operands for instruction: ${parts[0]}`);
            }
            if (["JMP","CAL","PGE"].includes(parts[0]) && parts.length !== 2) {
                throw new Error(`Invalid number of operands for instruction: ${parts[0]}`);
            }
            if (["LOD","STR","LDR","ADR","BRH","MOV"].includes(parts[0]) && parts.length !== 3) {
                throw new Error(`Invalid number of operands for instruction: ${parts[0]}`);
            }
            if (["ADD","SUB","NOR","AND","XOR"].includes(parts[0]) && parts.length !== 4) {
                throw new Error(`Invalid number of operands for instruction: ${parts[0]}`);
            }

            // check for correct operands
            // REG A
            if (["ADD","SUB","NOR","AND","XOR","MOV","LDR","ADR","PGE","LOD","STR"].includes(parts[0])) {
                if (registerSet[parts[1]] === undefined) {
                    throw new Error(`Invalid reg A for ${parts[0]} on line ${pc + 1}`);
                }
                instruction.push(registerSet[parts[1]]);
            }

            // REG B
            if (["ADD","SUB","NOR","AND","XOR","MOV"].includes(parts[0])) {
                if (registerSet[parts[2]] === undefined) {
                    throw new Error(`Invalid reg B for ${parts[0]} on line ${pc + 1}`);
                }
                instruction.push(registerSet[parts[2]]);
            }

            // REG C
            if (["ADD","SUB","NOR","AND","XOR"].includes(parts[0])) {
                if (registerSet[parts[3]] === undefined) {
                    throw new Error(`Invalid reg C for ${parts[0]} on line ${pc + 1}`);
                }
                instruction.push(registerSet[parts[3]]);
            }

            // VALUE & ADDRESS
            if (["STR", "LOD", "LDR", "ADR"].includes(parts[0])) {
                const value = parseInt(parts[2]);
                if (value < 0 || value >= 256) {
                    throw new Error(`Invalid value for ${parts[2]} on line ${pc + 1}`);
                }
                instruction.push(value);
            }

            // LABEL / TODO: actually make this work
            if (["JMP", "CAL", "BRH"].includes(parts[0])) {
                console.log(labels[parts[1]]);
                if (labels[parts[1]] === undefined) {
                    throw new Error(`Unknown label: ${parts[1]} on line ${pc + 1}`);
                }
                instruction.push(labels[parts[1]]);
            }

            // CONDITION
            if (parts[0] === "BRH") {
                if (parseInt(parts[2]) !== (parseInt(parts[2]) % (2 ** 2))) {
                    throw new Error(`Invalid condition for BRH on line ${pc + 1}`);
                }
                instruction.push(parseInt(parts[2]));
            }
            
            instructions.push(instruction);
        }

        // pass five, instruction to binary
        for (const instruction of instructions) {
            const opcode = instruction[0]; 
            const operand = instruction.slice(1).reduce((acc, val, index) => {
                const shift = (index === 3 || index === 2) ? 4 : 8;
                return (acc << shift) | val;
            }, 0);
            const binary = ((opcode << 12) | operand).toString(2).padStart(16, '0');
            assembled = new Uint16Array([...assembled, parseInt(binary, 2)]);
        }

        return assembled;
    }
}