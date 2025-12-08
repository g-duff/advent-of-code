import { readFile } from 'node:fs/promises';

const distance = (u: number[], v: number[]): number =>
  ((u[0] - v[0]) ** 2 + (u[1] - v[1]) ** 2 + (u[2] - v[2]) ** 2) ** 0.5;

const toCoordinates = (s: string) => s.split(',').map((x) => parseInt(x));

////////////
// Inputs //
////////////
// const N = 1000;
const infile = await readFile('data/day08.input', { encoding: 'utf-8' });

const rows = infile.trim().split('\n');

const R = rows.length;

const distances = new Array(R).fill([]).map((_) => new Array(R).fill(Infinity));

for (var i = 0; i < R; i++) {
  for (var j = 0; j < i; j++) {
    distances[i][j] = distance(toCoordinates(rows[i]), toCoordinates(rows[j]));
  }
}

const distancesWithCoords = distances
  .reduce(
    (acc, row, i) => [
      ...acc,
      ...row.reduce((acc, col, j) => [...acc, [col, i, j]], []),
    ],
    []
  )
  .filter(([d, ..._]) => d < Infinity)
  .sort(([da], [db]) => da - db);

var circuits: Set<number>[] = Array.from(
  { length: distances.length },
  (_, i) => new Set([i])
);

var i = 0;
var foundConnections = 0;
while (circuits.length > 2) {
  const [_, b1, b2] = distancesWithCoords[i];
  console.log(b1, b2);
  // find the sets
  const c1 = circuits.findIndex((c) => c.has(b1));
  const c2 = circuits.findIndex((c) => c.has(b2));

  // Already connected
  if (c1 == c2) {
    foundConnections++;
    i++;
    continue;
  }

  // Not connectd, so connect them!
  if (c1 != c2) {
    circuits.push(new Set([...circuits[c1], ...circuits[c2]]));
    circuits[c1] = new Set();
    circuits[c2] = new Set();
    circuits = circuits.filter((s) => s.size > 0);
    i++;
    foundConnections++;
  }
}

const lastElem = circuits.find((c) => c.size == 1)!;
const [_, b1, b2] = distancesWithCoords
  .slice(i, distancesWithCoords.length + 1)
  .find(([_, a, b]) => lastElem.has(a) || lastElem.has(b));

console.log(toCoordinates(rows[b1])[0] * toCoordinates(rows[b2])[0]);
