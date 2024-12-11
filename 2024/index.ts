import '@total-typescript/ts-reset';
import { solveDay11 } from './day11';

async function solve(
	cb: (type: 'basic' | 'actual', part: 1 | 2) => Promise<number>,
) {
	const result = await cb('actual', 2);
	console.log('The result is: ', result);
}

solve(solveDay11);
