import { readFile } from 'node:fs/promises';

const infile = await readFile('data/day02.input', { encoding: 'utf-8' });
const rows = infile.trim().split(',');

const pt1 = rows.reduce((pt1RunningTotal, row) => {
  const [first_id, last_id] = row.split('-').map((s) => parseInt(s));

  const rowTotal = Array(last_id - first_id + 1)
    .fill(first_id)
    .reduce((acc, id_init, id_offset) => {
      const this_id = id_offset + id_init;
      const str = String(this_id);

      const halfway = str.length / 2;
      const fullway = str.length + 1;

      const firsthalf = str.slice(0, halfway);
      const secondhalf = str.slice(halfway, fullway);
      const isInvalid = firsthalf == secondhalf;

      return isInvalid ? acc + this_id : acc;
    }, 0);

  return pt1RunningTotal + rowTotal;
}, 0);

console.log(`Part 1: ${pt1}`);

const pt2 = rows.reduce((pt2RunningTotal, row) => {
  const [b, e] = row.split('-').map((s) => parseInt(s));

  const invalidIdsInRow: Set<number> = new Set();
  for (var i = b; i <= e; i++) {
    const str = String(i);
    const n = str.length;

    for (var j = 1; j <= n / 2; j++) {
      if (n % j == 0) {
        const substrs = str.match(new RegExp(`.{1,${j}}`, 'g'))!;
        const [first, ...rest] = substrs;
        const isInvalid = rest.reduce(
          (acc, elem) => acc && first == elem,
          true
        );
        if (isInvalid) invalidIdsInRow.add(parseInt(str));
      }
    }
  }

  const rowTotal = Array.from(invalidIdsInRow).reduce(
    (acc, elem) => acc + elem,
    0
  );

  return pt2RunningTotal + rowTotal;
}, 0);

console.log(`Part 2: ${pt2}`);
