import { test, expect, describe } from 'bun:test';
import { moveRobot, parseRobot, solveDay14 } from '.';

test('correctly parses robot', () => {
	const robot = parseRobot('p=0,4 v=3,-3');
	expect(robot).not.toBeNull();
	expect(robot.position).toMatchObject({
		row: 4,
		col: 0,
	});
	expect(robot.velocity).toMatchObject({
		row: -3,
		col: 3,
	});
});

describe('move robots', () => {
	test('correctly moves first robot', () => {
		const robot = parseRobot('p=0,4 v=3,-3');
		const grid = Array.from({ length: 7 }, () =>
			Array.from({ length: 11 }, () => '.'),
		);
		moveRobot(robot, 1, grid);

		expect(robot.position.row).toBe(1);
		expect(robot.position.col).toBe(3);
		moveRobot(robot, 1, grid);
		expect(robot.position.row).toBe(5);
		expect(robot.position.col).toBe(6);
		moveRobot(robot, 1, grid);
		expect(robot.position.row).toBe(2);
		expect(robot.position.col).toBe(9);
		moveRobot(robot, 1, grid);
		expect(robot.position.row).toBe(6);
		expect(robot.position.col).toBe(1);
		moveRobot(robot, 1, grid);
		expect(robot.position.row).toBe(3);
		expect(robot.position.col).toBe(4);
		moveRobot(robot, 1, grid);
		expect(robot.position.row).toBe(0);
		expect(robot.position.col).toBe(7);
	});

	test('correctly moves second robot', () => {
		const robot = parseRobot('p=2,4 v=2,-3');
		const grid = Array.from({ length: 7 }, () =>
			Array.from({ length: 11 }, () => '.'),
		);
		moveRobot(robot, 1, grid);

		expect(robot.position.row).toBe(1);
		expect(robot.position.col).toBe(4);
		moveRobot(robot, 1, grid);
		expect(robot.position.row).toBe(5);
		expect(robot.position.col).toBe(6);
		moveRobot(robot, 1, grid);
		expect(robot.position.row).toBe(2);
		expect(robot.position.col).toBe(8);
		moveRobot(robot, 1, grid);
		expect(robot.position.row).toBe(6);
		expect(robot.position.col).toBe(10);
		moveRobot(robot, 1, grid);
		expect(robot.position.row).toBe(3);
		expect(robot.position.col).toBe(1);
	});
	test('correctly moves third robot', () => {
		const robot = parseRobot('p=7,6 v=-1,-3');
		const grid = Array.from({ length: 7 }, () =>
			Array.from({ length: 11 }, () => '.'),
		);
		moveRobot(robot, 1, grid);

		expect(robot.position.row).toBe(3);
		expect(robot.position.col).toBe(6);
		moveRobot(robot, 1, grid);
		expect(robot.position.row).toBe(0);
		expect(robot.position.col).toBe(5);
		moveRobot(robot, 1, grid);
		expect(robot.position.row).toBe(4);
		expect(robot.position.col).toBe(4);
		moveRobot(robot, 1, grid);
		expect(robot.position.row).toBe(3);
		expect(robot.position.col).toBe(1);
		moveRobot(robot, 1, grid);
		expect(robot.position.row).toBe(2);
		expect(robot.position.col).toBe(5);
		moveRobot(robot, 1, grid);
		expect(robot.position.row).toBe(1);
		expect(robot.position.col).toBe(2);
		moveRobot(robot, 1, grid);
		expect(robot.position.row).toBe(0);
		expect(robot.position.col).toBe(6);
		moveRobot(robot, 1, grid);
		expect(robot.position.row).toBe(10);
		expect(robot.position.col).toBe(3);
	});
});

test('find correct solution for basic part1', async () => {
	const result = await solveDay14('basic', 1);
	expect(result).toBe(12);
});
