import { test, expect, describe } from 'bun:test';
import { compute, solveDay17 } from '.';

describe('correctly computes', () => {
	test('check bst', () => {
		const comp = {
			registerA: 0,
			registerB: 0,
			registerC: 9,
			program: [2, 6],
			instructionPointer: 0,
			output: [],
		};
		compute(comp);
		expect(comp.registerB).toBe(1);
	});
	test('check output', () => {
		const comp = {
			registerA: 10,
			registerB: 0,
			registerC: 0,
			program: [5, 0, 5, 1, 5, 4],
			instructionPointer: 0,
			output: [],
		};
		compute(comp);
		expect(comp.output.join('')).toBe('012');
	});
	test('check output', () => {
		const comp = {
			registerA: 2024,
			registerB: 0,
			registerC: 0,
			program: [0, 1, 5, 4, 3, 0],
			instructionPointer: 0,
			output: [],
		};
		compute(comp);
		expect(comp.output.join('')).toBe('42567777310');
		expect(comp.registerA).toBe(0);
	});
	test('check bxl', () => {
		const comp = {
			registerA: 0,
			registerB: 29,
			registerC: 0,
			program: [1, 7],
			instructionPointer: 0,
			output: [],
		};
		compute(comp);
		expect(comp.registerB).toBe(26);
	});
	test('check bxc', () => {
		const comp = {
			registerA: 0,
			registerB: 2024,
			registerC: 43690,
			program: [4, 0],
			instructionPointer: 0,
			output: [],
		};
		compute(comp);
		expect(comp.registerB).toBe(44354);
	});
	test('complex loop with large numbers', () => {
		const comp = {
			registerA: 24576, // 2^13 + 2^14
			registerB: 0,
			registerC: 0,
			program: [
				0,
				2, // A = A/4
				5,
				4, // Output A%8
				2,
				4, // B = 4
				7,
				5, // C = A/2^B
				1,
				6, // B = B XOR 6
				3,
				0, // Jump to start if A != 0
			],
			instructionPointer: 0,
			output: [],
		};
		compute(comp);
		expect(comp.registerA).toBe(0);
		// This should create a deterministic sequence as A is repeatedly divided
	});
	test('register interaction', () => {
		const comp = {
			registerA: 1024,
			registerB: 16,
			registerC: 8,
			program: [
				2,
				6, // Set B to A%C
				7,
				4, // Store A/2^B in C
				4,
				0, // XOR B with C
				5,
				5, // Output B%8
			],
			instructionPointer: 0,
			output: [],
		};
		compute(comp);
		expect(comp.registerB).toBe(0);
		expect(comp.registerC).toBe(0);
		expect(comp.output.join()).toBe('0');
	});
	test('jump loop test', () => {
		const comp = {
			registerA: 32,
			registerB: 0,
			registerC: 0,
			program: [0, 1, 5, 4, 3, 0], // Similar pattern to your working test
			instructionPointer: 0,
			output: [],
		};
		compute(comp);
		// Should divide by 2 repeatedly and output remainders
		expect(comp.output.join('')).toBe('004210');
		expect(comp.registerA).toBe(0);
	});
	test('large division with adv', () => {
		const comp = {
			registerA: 16384, // 2^14
			registerB: 0,
			registerC: 0,
			program: [0, 3, 5, 4], // divide by 2^3=8, then output A%8
			instructionPointer: 0,
			output: [],
		};
		compute(comp);
		expect(comp.registerA).toBe(2048); // 16384/8
		expect(comp.output.join()).toBe('0');
	});
});

test('find correct solution for basic part1', async () => {
	const result = await solveDay17('basic', 1);
	expect(result).toBe(4635635210);
});

test('find correct solution for basic part2', async () => {
	const result = await solveDay17('basic_new', 2);
	expect(result).toBe(117440);
});
