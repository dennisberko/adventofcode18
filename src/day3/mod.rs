use std::fs::read_to_string;

extern crate regex;
use self::regex::Regex;

const SIZE: usize = 1000;

struct Rect {
	id: String,
	x: u16,
	y: u16,
	width: u16,
	height: u16,
}

fn get_rect(line: &str) -> Rect {
	let re = Regex::new(r"#(\d+) @ (\d+),(\d+): (\d+)x(\d+)").unwrap();
	let caps = re.captures(&line).unwrap();

	let id = caps.get(1).map_or("", |m| m.as_str());
	let x_str = caps.get(2).map_or("", |m| m.as_str());
	let y_str = caps.get(3).map_or("", |m| m.as_str());
	let width_str = caps.get(4).map_or("", |m| m.as_str());
	let height_str = caps.get(5).map_or("", |m| m.as_str());

	let x: u16 = x_str.parse().unwrap();
	let y: u16 = y_str.parse().unwrap();
	let width: u16 = width_str.parse().unwrap();
	let height: u16 = height_str.parse().unwrap();

	Rect {
		id: id.to_string(),
		x: x,
		y: y,
		width: width,
		height: height,
	}
}

fn construct_overlap_grid(rects: &Vec<Rect>) -> Vec<Vec<u16>> {
	let mut cols = Vec::new();

	for _col in 0..SIZE {
		cols.push(vec![0; SIZE]);
	}

	for rect in rects {
		let end_x = rect.x + rect.width;
		let end_y = rect.y + rect.height;

		for x in rect.x..end_x {
			for y in rect.y..end_y {
				cols[x as usize][y as usize] += 1
			}
		}
	}

	cols
}

fn part1(overlap_grid: &Vec<Vec<u16>>) -> i32 {
	let mut overlapping_squares = 0;

	for x in 0..SIZE {
		for y in 0..SIZE {
			if overlap_grid[x][y] >= 2 {
				overlapping_squares += 1
			}
		}
	}

	overlapping_squares
}

fn part2(overlap_grid: &Vec<Vec<u16>>, rects: &Vec<Rect>) -> String {
	'rect_loop: for rect in rects {
		let end_x = rect.x + rect.width;
		let end_y = rect.y + rect.height;

		for x in rect.x..end_x {
			for y in rect.y..end_y {
				let square = overlap_grid[x as usize][y as usize];

				if square > 1 {
					continue 'rect_loop;
				}
			}
		}

		return rect.id.clone();
	}

	String::new()
}

pub fn main() {
	let filename = "src/day3/input";

	let contents = read_to_string(filename).expect("Error reading file");
	let lines: Vec<&str> = contents.lines().collect();

	let rects = lines.into_iter().map(|line| get_rect(line)).collect();

	let overlap_grid = construct_overlap_grid(&rects);

	print!("part1: {}\n", part1(&overlap_grid));
	print!("part2: {}", part2(&overlap_grid, &rects));
}
