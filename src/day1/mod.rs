use std::collections::HashMap;

fn get_new_freq(freq: i32, line: &str) -> i32 {
	let mut chars = line.chars();
	let operation = chars.next();
	let delta_str: String = chars.collect();
	let delta: i32 = delta_str.parse().unwrap();

	if operation == Some('+') {
		freq + delta
	} else {
		freq - delta
	}
}

fn part1(lines: &Vec<&str>) -> i32 {
	let mut freq: i32 = 0;

	for line in lines {
		freq = get_new_freq(freq, line);
	}

	freq
}

fn part2(lines: &Vec<&str>) -> i32 {
	let mut freq: i32 = 0;
	let mut freqs = HashMap::new();

	let mut i = 0;
	loop {
		let count = freqs.entry(freq).or_insert(0);
		*count += 1;

		if *count >= 2 {
			return freq;
		}

		i %= lines.len();

		freq = get_new_freq(freq, &lines[i]);

		i += 1;
	}
}

pub fn main() {
	let filename = "src/day1/input";

	let contents = std::fs::read_to_string(filename).expect("Something went wrong reading the file");
	let lines: Vec<&str> = contents.lines().collect();

	print!("part1: {}\n", part1(&lines));
	print!("part2: {}", part2(&lines));
}
