fn main() {
	use std::fs::File;
	use std::io::{BufRead, BufReader};

	let mut sum: i64 = 0;

	let f = File::open("input.txt").unwrap();
	let reader = BufReader::new(f);

	for line in reader.lines() {
		let s = line.unwrap();

		let length = s.len();
		let mut left = 0;
		let mut digits = String::new();

		for i in 0..=11 {
			let mut max = '\0';
			let mut maxj = 0;
			let right = length - (11 - i);
			let slice = &s[left..right];
			for (j, c) in slice.chars().enumerate() {
				if c > max {
					max = c;
					maxj = left + j;
				}
			}
			digits.push(max);
			left = maxj + 1;
		}
		println!("{}", digits);
		sum += digits.parse::<i64>().unwrap();
	}

	println!("Sum: {}", sum);
}
