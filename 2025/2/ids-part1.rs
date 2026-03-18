fn is_less_or_eq(a: &str, b: &str) -> bool {
	if a.len() < b.len() {
		return true;
	} else if a.len() > b.len() {
		return false;
	}

	for (c1, c2) in a.chars().zip(b.chars()) {
		if c1 < c2 {
			return true;
		} else if c1 > c2 {
			return false;
		}
	}
	true
}

fn inc(s: &str) -> String {
	//println!("inc: {}", s);

	let mut v: Vec<char> = vec![];
	let mut did_inc: bool = false;
	for c in s.chars().rev() {
		if did_inc {
			v.push(c);
		} else if c == '9' {
			v.push('0');
		} else {
			v.push(std::char::from_u32(u32::from(c) + 1).unwrap());
			did_inc = true;
		}
	}

	if !did_inc {
		v.push('1');
	}

	v.reverse();
	return v.into_iter().collect();
		
}

fn do_range(low: &str, high: &str) -> i64 {
	println!("Doing range {}-{}", low, high);

	let mut sum: i64 = 0;

	let mut pattern: String = if low.len() % 2 == 0 {
		low[0..low.len()/2].to_owned()
	} else {
		"1".to_owned() + &"0".repeat(low.len()/2)
	};

	loop {
		let s = pattern.clone() + &pattern.clone();
		if !is_less_or_eq(&s, high) {
			println!("\tTerminating because {} > {}.", &s, high);
			return sum;
		}

		if is_less_or_eq(low, &s) {
			println!("\tFound {}", s);
			sum += &s.parse().unwrap();
		}

		pattern = inc(&pattern);
	}
}

fn main() {
	use std::fs::File;
	use std::io::{BufRead, BufReader};

	let mut sum: i64 = 0;

	let f = File::open("input.txt").unwrap();
	let reader = BufReader::new(f);

	for line in reader.lines() {
		let s = line.unwrap();
		for range in s.split(',') {
			let (low, high) = range.split_once('-').unwrap();
			sum += do_range(low, high);
		}
	}

	println!("Sum: {}", sum);
}
