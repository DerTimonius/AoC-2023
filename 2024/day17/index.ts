type OperandType = 'literal' | 'combo';
type Computer = {
	registerA: number;
	registerB: number;
	registerC: number;
	program: number[];
	instructionPointer: number;
	output: number[];
};

type Instruction = {
	operand: OperandType;
	func: (a: number, comp: Computer) => void;
};

export async function solveDay17(
	type: 'actual' | 'basic' | 'basic_new',
	part: 1 | 2,
): Promise<number> {
	const file = Bun.file(`${import.meta.dir}/${type}.txt`);
	const text = await file.text();
	const computer = parseComputer(text);

	return part === 1 ? solvePart1(computer) : solvePart2(computer);
}

function parseComputer(text: string): Computer {
	const [regParts, progParts] = text.split('\n\n');
	const [regA, regB, regC] = regParts
		.split('\n')
		.map((line) => line.match(/\d+/));
	const parsedProg = progParts.match(/\d+/g);

	if (!regA || !regB || !regC || !parsedProg) {
		throw new Error(`could not parse text: ${text}`);
	}

	return {
		registerA: Number.parseInt(regA as unknown as string),
		registerB: Number.parseInt(regB as unknown as string),
		registerC: Number.parseInt(regC as unknown as string),
		program: parsedProg.map((x) => Number.parseInt(x)),
		instructionPointer: 0,
		output: [],
	};
}

function getInstruction(num: number): Instruction {
	switch (num) {
		case 0:
			return {
				operand: 'combo',
				func: (a: number, c: Computer) => {
					c.registerA = Math.floor(c.registerA / 2 ** a);
				},
			};
		case 1:
			return {
				operand: 'literal',
				func: (a: number, c: Computer) => {
					c.registerB = c.registerB ^ a;
				},
			};
		case 2:
			return {
				operand: 'combo',
				func: (a: number, c: Computer) => {
					c.registerB = a % 8;
				},
			};
		case 3:
			return {
				operand: 'literal',
				func: (a: number, c: Computer) => {
					if (c.registerA === 0) return;

					c.instructionPointer = a - 2;
				},
			};
		case 4:
			return {
				operand: 'combo',
				func: (_a: number, c: Computer) => {
					c.registerB = c.registerB ^ c.registerC;
				},
			};
		case 5:
			return {
				operand: 'combo',
				func: (a: number, c: Computer) => {
					c.output.push(a % 8);
				},
			};
		case 6:
			return {
				operand: 'combo',
				func: (a: number, c: Computer) => {
					c.registerB = Math.floor(c.registerA / 2 ** a);
				},
			};
		case 7:
			return {
				operand: 'combo',
				func: (a: number, c: Computer) => {
					c.registerC = Math.floor(c.registerA / 2 ** a);
				},
			};

		default:
			throw new Error('invalid instruction');
	}
}

function getOperand(num: number, type: OperandType, comp: Computer) {
	if (type === 'literal') return num;

	switch (num) {
		case 0:
		case 1:
		case 2:
		case 3:
			return num;
		case 4:
			return comp.registerA;
		case 5:
			return comp.registerB;
		case 6:
			return comp.registerC;

		default:
			throw new Error('invalid operand');
	}
}

export function compute(comp: Computer) {
	while (comp.instructionPointer + 1 < comp.program.length) {
		const instruction = getInstruction(comp.program[comp.instructionPointer]);
		const operand = getOperand(
			comp.program[comp.instructionPointer + 1],
			instruction.operand,
			comp,
		);
		instruction.func(operand, comp);

		comp.instructionPointer += 2;
	}
}

function runProgram(initialA: number, comp: Computer): string {
	const testComp = {
		...comp,
		registerA: initialA,
		output: [],
		instructionPointer: 0,
	};
	compute(testComp);
	return testComp.output.join(',');
}

function solvePart1(comp: Computer): number {
	compute(comp);
	const output = comp.output.join(',');

	return Number.parseInt(output.replaceAll(',', ''));
}

function solvePart2(comp: Computer): number {
	const queue: { result: string; len: number }[] = [{ result: '', len: 0 }];

	while (queue.length > 0) {
		const current = queue.shift()!;
		if (current.len === comp.program.length) {
			const finalValue = Number.parseInt(current.result, 2);
			return finalValue;
		}

		const baseNumber = Number.parseInt(`${current.result}000`, 2);
		const maxNumber = Number.parseInt(`${current.result}111`, 2);

		const expectedOutput = comp.program.slice(-1 * (current.len + 1)).join(',');
		for (let a = baseNumber; a <= maxNumber; a++) {
			const output = runProgram(a, comp);
			if (output === expectedOutput) {
				queue.push({
					result: a.toString(2),
					len: current.len + 1,
				});
			}
		}
	}

	throw new Error('No solution found');
}
