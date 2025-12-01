import { readFile } from "node:fs/promises";


const infile = await readFile("data/day01.input", {encoding: "utf-8"})
const rows = infile.trim().split('\n');


// Horrible
var state = 50;

var pt1 = 0;
const digits = rows.map((row) => {
	const [dir, ...countStr] = row;
	const count = parseInt(countStr.join(''));

	switch (dir) {
		case "L":
			state -= count;
			break;
		case "R":
			state += count;
			break;
		default:	
			throw new Error("Neither L nor R");
	}

	if (state < 0) {
		state = (state%100) + 100;
	}
	if (state > 99) {
		state = (state%100);
	}

	if (state == 0) pt1++;
	return state;
});

console.table(digits);

console.log(`Part 1: ${pt1}`);
