import { readFile } from 'node:fs/promises';

const infile = await readFile('data/day05.input', { encoding: 'utf-8' });

const [rangesRaw, idsRaw] = infile.trim().split('\n\n');

const ranges = rangesRaw
  .split('\n')
  .map((row) => row.split('-').map((x) => parseInt(x)));

const ids = idsRaw.split('\n').map((x) => parseInt(x));

const pt1 = ids
  .map((id) => ranges.map(([l, h]) => l <= id && id <= h))
  .map((inRanges) => inRanges.reduce((acc, b) => acc || b, false))
  .filter((inAnyRange) => inAnyRange).length;

console.log(pt1);
