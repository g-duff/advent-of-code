import { readFile } from 'node:fs/promises';
import { defined } from '#src/index.ts';

const infile = await readFile('data/day07.input', { encoding: 'utf-8' });

const [startingRow, ...rows] = infile
  .trim()
  .split('\n')
  .map((row) => row.split(''));

const C = startingRow.length;

////////////
// Part 1 //
////////////

var pt1 = 0;
const beamIdxs = new Set([startingRow.findIndex((cell) => cell == 'S')]);
for (const row of rows) {
  const validSplitters = row
    .map((elem, idx) => (elem == '^' && beamIdxs.has(idx) ? idx : undefined))
    .filter(defined);

  pt1 += validSplitters.length;
  validSplitters.forEach((s) => beamIdxs.delete(s));
  for (const s of validSplitters) {
    beamIdxs.add(s - 1);
    beamIdxs.add(s + 1);
  }
}
console.log(pt1);

////////////
// Part 2 //
////////////

const beamCols: number[] = Array(C).fill(0);
const startIndex = startingRow.findIndex((cell) => cell == 'S');
beamCols[startIndex] = 1;

for (const row of rows) {
  const validSplitters = row
    .map((elem, idx) => (elem == '^' && beamCols[idx] > 0 ? idx : undefined))
    .filter(defined);

  for (const s of validSplitters) {
    const n = beamCols[s];
    beamCols[s] = 0;

    const nL = beamCols[s - 1];
    beamCols[s - 1] = nL + n;

    const nR = beamCols[s + 1];
    beamCols[s + 1] = nR + n;
  }
}

const pt2 = beamCols.entries().reduce((acc, [_, val]) => acc + val, 0);
console.log(pt2);
