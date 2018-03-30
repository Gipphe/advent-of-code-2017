fn find_largest(input: &Vec<u32>) -> usize {
	let mut largest = 0;
	let mut largest_pos = 0;
	for (i, x) in input.into_iter().enumerate() {
		if x > &largest {
			largest_pos = i;
			largest = *x;
		}
	}
	largest_pos
}

fn redistribute(coll: Vec<u32>, start: usize) -> Vec<u32> {
	let mut coll = coll;
	let mut val = coll[start];
	coll[start] = 0;
	let mut i = start + 1;
	while val > 0 {
		i = i % coll.len();
		coll[i] += 1;
		val -= 1;
		i += 1;
	}
	coll
}

fn balance_banks(banks: Vec<u32>) -> Vec<u32> {
	let pos = find_largest(&banks);
	redistribute(banks, pos)
}

fn steps_until_infinite_loop(input: &str) -> u32 {
	let mut steps = 0;
	let mut blocks = input.split_whitespace()
		.map(|x| x.parse::<u32>().expect("Unable to parse number"))
		.collect::<Vec<u32>>();
	let mut snapshots: Vec<Vec<u32>> = Vec::new();

	'outer: loop {
		steps += 1;
		blocks = balance_banks(blocks);
		for snapshot in &snapshots {
			if &blocks == snapshot {
				break 'outer;
			}
		}
		snapshots.push(blocks.clone());
	}

	steps
}

fn cycles_between_loops(input: &str) -> u32 {
	let mut blocks = input.split_whitespace()
		.map(|x| x.parse::<u32>().expect("Unable to parse number"))
		.collect::<Vec<u32>>();
	let mut snapshots: Vec<Vec<u32>> = Vec::new();

	'outer: loop {
		blocks = balance_banks(blocks);
		for snapshot in &snapshots {
			if &blocks == snapshot {
				break 'outer;
			}
		}
		snapshots.push(blocks.clone());
	}
	let i = (&snapshots).into_iter().position(|a| a == &blocks)
		.expect("Error in locating snapshot");

	(snapshots.len() - i) as u32
}

pub fn main() {
	let input = "5	1	10	0	1	7	13	14	3	12	8	10	7	12	0	6";
	let first = steps_until_infinite_loop(&input);
	let second = cycles_between_loops(&input);

	assert_eq!(first, 5042, "Day 6-1 is incorrect: {}", first);
	assert_eq!(second, 1086, "Day 6-2 is incorrect: {}", second);
	println!("Day 6-1: {}", first);
	println!("Day 6-2: {}", second);
}
