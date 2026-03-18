fn count_neighbors(a: &Vec<Vec<char>>, i: usize, j: usize) -> i64 {
	let mut n = 0;
	let directions = [
		(-1, -1),
		(-1, 0),
		(-1, 1),
		(0, -1),
		(0, 1),
		(1, -1),
		(1, 0),
		(1, 1)];

	for (di, dj) in directions.iter() {
		let ei = i as isize + di;
		let ej = j as isize + dj;
		if ei >= 0 && ei < a.len() as isize && ej >= 0 && ej < a[ei as usize].len() as isize {
			if a[ei as usize][ej as usize] == '@' {
				n += 1;
			}
		}
	}

	return n;
}

fn doit(a: &mut Vec<Vec<char>>) -> i64 {
	let mut sum = 0;
	for i in 0..a.len() {
		for j in 0..a[i].len() {
			if a[i][j] == '@' {
				let n = count_neighbors(&a, i, j);
				if n < 4 {
					sum += 1;
					a[i][j] = 'x';
				}
			}
		}
	}
	return sum;
}

fn main() {
	use std::fs::File;
	use std::io::{BufRead, BufReader};

	let mut sum: i64 = 0;

	let f = File::open("input.txt").unwrap();
	let reader = BufReader::new(f);

	let mut a: Vec<Vec<char>> = Vec::new();

	for line in reader.lines() {
		let s = line.unwrap();
		a.push(s.chars().collect());
	}

	loop {
		let n = doit(&mut a);
		sum += n;
		if n == 0 {
			break;
		}
	}

	println!("Sum: {}", sum);
}
