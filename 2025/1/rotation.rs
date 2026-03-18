fn main() {
	use std::fs::File;
	use std::io::{BufRead, BufReader};

	let mut pos: i32 = 50;
	let mut zeroes = 0;
	
	let f = File::open("input.txt").unwrap();
	let reader = BufReader::new(f);

	for line in reader.lines() {
		let s = line.unwrap();
		let dist: &i32 = &s[1..].parse().unwrap();

		if &s[0..1] == "L" {
			pos = (100 - pos) % 100;	// invert numbering
			pos += dist;
			zeroes += pos / 100;
			pos %= 100;
			pos = (100 - pos) % 100;	// invert numbering back
		} else if &s[0..1] == "R" {
			pos += dist;
			zeroes += pos / 100;
			pos %= 100;
		} else {
			panic!("invalid direction");
		}

		println!("got direction {} and distance {}, new pos is {}, zeroes is {}", &s[0..1], dist, pos, zeroes);
	}

	println!("zeroes: {}", zeroes);
}
