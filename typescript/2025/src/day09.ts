import { readFile } from 'node:fs/promises';

const calculateArea: (c1: [number, number], c2: [number, number]) => number = (
  [x1, y1],
  [x2, y2]
) => (Math.abs(x1 - x2) + 1) * (Math.abs(y1 - y2) + 1);

const infile = await readFile('data/day09.input', { encoding: 'utf-8' });

const coords = infile
  .trim()
  .split('\n')
  .map((row) => row.split(',').map((x) => parseInt(x))) as [number, number][];

const areasWithCoordIdxs: [number, [number, number], [number, number]][] = [];
for (var i = 0; i < coords.length; i++) {
  for (var j = 0; j < i; j++) {
    const area = calculateArea(coords[i], coords[j]);
    areasWithCoordIdxs.push([area, coords[i], coords[j]]);
  }
}

const sortedAreasWithCoords = areasWithCoordIdxs.sort(([a], [b]) => b - a);

const pt1 = sortedAreasWithCoords[0][0];
console.log(`Part 1: ${pt1}`);

// Did part 2 in python
