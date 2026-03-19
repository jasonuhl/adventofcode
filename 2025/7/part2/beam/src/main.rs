fn main() {
	use std::fs::File;
	use std::io::{prelude::*, BufReader};

	let f = File::open("input.txt").unwrap();
	let reader = BufReader::new(f);

	#[derive(Debug)]
	struct Cell {
		c: char,
		n: i64,
	}

	let mut a: Vec<Vec<Cell>> = Vec::new();

	for line in reader.lines() {
		let s = line.unwrap();
		let cell_vec: Vec<Cell> = s.chars().map(|c| Cell { c: c, n: 0 }).collect();
		a.push(cell_vec);
	}

	let mut splitcount = 0;

	for row in 0..a.len()-1 {
		for col in 0..a[row].len() {
			if a[row][col].c == 'S' {
				a[row][col].n = 1;
			}

			if a[row][col].c == 'S' || a[row][col].c == '|' {
				if a[row + 1][col].c == '^' {
					splitcount += 1;

					a[row + 1][col - 1].c = '|';
					a[row + 1][col - 1].n += a[row][col].n;

					a[row + 1][col + 1].c = '|';
					a[row + 1][col + 1].n += a[row][col].n;

				} else {
					a[row + 1][col].c = '|';
					a[row + 1][col].n += a[row][col].n;
				}
			}
		}
	}

	for row in 0..a.len() {
		println!("{}", a[row].iter().map(|cell| cell.c).collect::<String>());
	}

	println!("{}", splitcount);

	for row in 0..a.len() {
		println!("{}", a[row].iter().map(|cell| format!("{:x}", cell.n)).collect::<String>());
	}

	let timelines: i64 = a.last().unwrap().iter().map(|cell| cell.n).sum();
	println!("{}", timelines);

}
