fn roll_column_north(a: &mut Vec<Vec<char>>, colidx: usize) {
    let mut potential_dest: usize = 0;
    for rowidx in 0..a.len() {
        let c = a[rowidx][colidx];
        println!("Column {}: Considering row {}: {} (potential_dest == {})", colidx, rowidx, c, potential_dest);
        if c == '#' {
            potential_dest = rowidx;
        } else if c == 'O' {
            while a[potential_dest][colidx] != '.' && potential_dest < rowidx {
                potential_dest += 1;
            }
            if a[potential_dest][colidx] == '.' {
                println!("Column {}: Moving O from {} to {}", colidx, rowidx, potential_dest);
                a[potential_dest][colidx] = 'O';
                a[rowidx][colidx] = '.';
            }
        }
    }
}

fn roll_north(a: &mut Vec<Vec<char>>) {
    for colidx in 0..a[0].len() {
        roll_column_north(a, colidx);
    }
}

fn pretty_print(a: &Vec<Vec<char>>) {
    for row in a {
        for col in row {
            print!("{}", col);
        }
        println!("");
    }
}

fn total_load(a: &Vec<Vec<char>>) {
    let mut total = 0;
    for rowidx in 0..a.len() {
        for colidx in 0..a[0].len() {
            if a[rowidx][colidx] == 'O' {
                total += a.len() - rowidx;
            }
        }
    }
    println!("Total load: {}", total);
}

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

    roll_north(&mut a);
    pretty_print(&a);
    total_load(&a);
}
