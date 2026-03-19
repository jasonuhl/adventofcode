fn main() {
	use std::fs::File;
	use std::io::{prelude::*, BufReader};

	let f = File::open("input.txt").unwrap();
	let reader = BufReader::new(f);

	let mut a: Vec<Vec<char>> = vec![];

	for line in reader.lines() {
		let s = line.unwrap();
		let char_vec: Vec<char> = s.chars().collect();
		a.push(char_vec);
	}

	let mut splitcount = 0;

	for row in 0..a.len()-1 {
		for col in 0..a[row].len() {
			if a[row][col] == 'S' || a[row][col] == '|' {
				if a[row + 1][col] == '^' {
					splitcount += 1;
					if a[row + 1][col - 1] == '.' {
						a[row + 1][col - 1] = '|';
					}
					if a[row + 1][col + 1] == '.' {
						a[row + 1][col + 1] = '|';
					}
				} else if a[row + 1][col] == '.' {
					a[row + 1][col] = '|';
				}
			}
		}
	}

	for row in 0..a.len() {
		println!("{}", a[row].iter().collect::<String>());
	}

	println!("{}", splitcount);
}
