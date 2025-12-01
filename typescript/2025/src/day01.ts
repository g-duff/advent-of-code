import { readFile } from "node:fs/promises";


const infile = await readFile("data/day01.input", {encoding: "utf-8"})
const rows = infile.trim().split('\n');


// Horrible
var state = 50;

var pt1 = 0;
var pt2 = 0;
rows.forEach((row) => {
	const [dir, ...countStr] = row;
	const count = parseInt(countStr.join(''));

	if (dir == "R")
		for (var n=0; n<count;n++){
			state += 1;
			if (state % 100 == 0) pt2++;
		}
	else 	
		for (var n=0; n<count;n++){
			state -= 1;
			if (state % 100 == 0) pt2++;
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

console.log(`Part 1: ${pt1}`);
console.log(`Part 2: ${pt2}`);
