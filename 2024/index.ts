import '@total-typescript/ts-reset';
import { solveDay10 } from './day10';

async function solve(
	cb: (type: 'basic' | 'actual', part: 1 | 2) => Promise<number>,
) {
	const result = await cb('actual', 2);
	console.log('The result is: ', result);
}

solve(solveDay10);
