use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::read_to_string;

fn part1(lines: &Vec<&str>) -> i32 {
	let mut num_twos: i32 = 0;
	let mut num_threes: i32 = 0;

	for line in lines {
		let mut char_freqs: HashMap<char, i32> = HashMap::new();

		for chr in line.chars() {
			let count = char_freqs.entry(chr).or_insert(0);
			*count += 1;
		}

		let values = char_freqs.values();
		let values_set: HashSet<&i32> = values.collect();

		if values_set.contains(&&2) {
			num_twos += 1
		}

		if values_set.contains(&&3) {
			num_threes += 1
		}
	}

	num_twos * num_threes
}

fn part2(lines: &Vec<&str>) -> String {
	let mut min_diff: i32 = 1000;
	let mut common_letters = String::new();

	let mut existing_ids: Vec<&str> = Vec::new();

	for line in lines {
		for id in &existing_ids {
			let mut id_chars = id.chars();
			let mut num_diffs = 0;
			let mut common_letters_tmp = String::new();

			for line_char in line.chars() {
				let id_char = id_chars.next();

				match id_char {
					Some(chr) => {
						if line_char != chr {
							num_diffs += 1
						} else {
							common_letters_tmp.push(chr)
						}
					}
					None => num_diffs += 1,
				}

				if num_diffs > min_diff {
					break;
				}
			}

			if num_diffs < min_diff {
				min_diff = num_diffs;
				common_letters = common_letters_tmp.clone();
			}
		}

		existing_ids.push(line);
	}

	common_letters
}

pub fn main() {
	let filename = "src/day2/input";

	let contents = read_to_string(filename).expect("Error reading file");
	let lines: Vec<&str> = contents.lines().collect();

	print!("part1: {}\n", part1(&lines));
	print!("part2: {}", part2(&lines));
}
