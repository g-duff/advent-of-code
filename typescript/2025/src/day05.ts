import { readFile } from 'node:fs/promises';

const infile = await readFile('data/day05.input', { encoding: 'utf-8' });

const [rangesRaw, idsRaw] = infile.trim().split('\n\n');
const ranges = rangesRaw
  .split('\n')
  .map((row) => row.split('-').map((x) => parseInt(x)));
const ids = idsRaw.split('\n').map((x) => parseInt(x));

////////////
// Part 1 //
////////////

const pt1 = ids
  .map((id) => ranges.map(([l, h]) => l <= id && id <= h))
  .map((inRanges) => inRanges.reduce((acc, b) => acc || b, false))
  .filter((inAnyRange) => inAnyRange).length;

console.log(pt1);

////////////
// Part 2 //
////////////

// Coalesce ranges
// if start is within a range
// 	if end is outside the range
// 		extend the range
// 	if end is within the range
// 		do nothing
// if start is outside the range
// 	add a new range to coalesced
const coalesce: (ranges: number[][], coalesced: number[][]) => number[][] = (
  [thisRange, ...nextRanges],
  coalesced
) => {
  if (!thisRange) return coalesced;

  const [[coalescedStart, coalescedEnd], ...otherCoalesced] = coalesced;
  const [rangeStart, rangeEnd] = thisRange;

  if (rangeStart <= coalescedEnd)
    return coalesce(nextRanges, [
      [coalescedStart, Math.max(rangeEnd, coalescedEnd)],
      ...otherCoalesced,
    ]);
  else
    return coalesce(nextRanges, [
      [rangeStart, rangeEnd],
      [coalescedStart, coalescedEnd],
      ...otherCoalesced,
    ]);
};

const sortedRanges = ranges.sort(([aStart], [bStart]) => aStart - bStart);
const [first, ...rest] = sortedRanges;
const pt2 = coalesce(rest, [first]).reduce(
  (acc, [start, end]) => acc + end - start + 1,
  0
);
console.log(pt2);
