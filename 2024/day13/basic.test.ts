import { test, expect } from 'bun:test';
import { parseBlock, solveDay13 } from '.';

test('correctly parses game', () => {
	const game = parseBlock(`Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400`);
	expect(game).not.toBeNull();
	expect(game.buttonA).toMatchObject({
		x: 94,
		y: 34,
	});
	expect(game.buttonB).toMatchObject({
		x: 22,
		y: 67,
	});
	expect(game.prize).toMatchObject({
		x: 8400,
		y: 5400,
	});
});

test('find correct solution for basic part1', async () => {
	const result = await solveDay13('basic', 1);
	expect(result).toBe(480);
});
