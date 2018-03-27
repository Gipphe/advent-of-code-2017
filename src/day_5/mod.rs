mod input;

fn steps_to_escape(input: &str) -> u32 {
	let mut steps = 0;
	let tape = input.lines().collect::<Vec<&str>>();
	let tape = tape.into_iter();
	let tape = tape.map(|x| x.parse::<i32>().expect("Could not parse to number"));
	let mut tape = tape.collect::<Vec<i32>>();
	let mut pointer: i32 = 0;
	let end = tape.len() as i32;
	while pointer >= 0 && pointer < end {
		let new_pointer = pointer + tape[pointer as usize];
		tape[pointer as usize] += 1;
		pointer = new_pointer;
		steps += 1;
	}
	steps
}
fn steps_to_escape_alt(input: &str) -> u32 {
	let mut steps = 0;
	let tape = input.lines().collect::<Vec<&str>>();
	let tape = tape.into_iter();
	let tape = tape.map(|x| x.parse::<i32>().expect("Could not parse to number"));
	let mut tape = tape.collect::<Vec<i32>>();
	let mut pointer: i32 = 0;
	let end = tape.len() as i32;
	while pointer >= 0 && pointer < end {
		let tape_val = tape[pointer as usize];
		let new_pointer = pointer + tape_val;
		if tape_val >= 3 {
			tape[pointer as usize] -= 1;
		} else {
			tape[pointer as usize] += 1;
		}
		pointer = new_pointer;
		steps += 1;
	}
	steps
}

pub fn main() {
	let first = steps_to_escape(&input::INPUT);
	let second = steps_to_escape_alt(&input::INPUT);
	println!("Day 5-1: {}", first);
	println!("Day 5-2: {}", second);
}
