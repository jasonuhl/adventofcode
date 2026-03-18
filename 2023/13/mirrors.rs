type Pattern = Vec<Vec<char>>;

fn find_reflection(a: &Pattern, direction: &str) -> u64 {
    use std::convert::TryFrom;

    for rowidx in 0..a.len() - 1 {
        if a[rowidx] == a[rowidx + 1] {
            let mut mirrortop = rowidx;
            let mut mirrorsize = 1;
            while mirrortop > 0 && rowidx + mirrorsize + 1 < a.len() && a[mirrortop - 1] == a[rowidx + mirrorsize + 1] {
                mirrortop -= 1;
                mirrorsize += 1;
            }

            // It only counts if it goes up to an edge.
            if mirrortop == 0 || rowidx + mirrorsize + 1 == a.len() {
                println!("found {} reflection: {} - {} of size {}", direction, rowidx, rowidx + 1, mirrorsize);
                let mut val: u64 = u64::try_from(rowidx).unwrap() + 1;
                if direction == "horizontal" {
                    val *= 100;
                }
                println!("value: {}", val);
                return val;
            }
        }
    }
    return 0;
}

fn find_horizontal_reflection(a: &Pattern) -> u64 {
    return find_reflection(&a, "horizontal");
}

fn rotate_pattern(a: &Pattern) -> Pattern {
    let mut cols: Pattern = vec![];
    for i in 0..a[0].len() {
        let mut col = vec![];
        for row in a.iter() {
            col.push(row[i]);
        }
        cols.push(col);
    }
    return cols;
}

fn find_vertical_reflection(a: &Pattern) -> u64 {
    let cols = rotate_pattern(a);
    return find_reflection(&cols, "vertical");
}

fn row_edit_distance(a: &Vec<char>, b: &Vec<char>) -> u64 {
    if a.len() != b.len() {
        panic!("mismatched lengths");
    }

    let mut edit_distance = 0;
    for i in 0..a.len() {
        if a[i] != b[i] {
            edit_distance += 1;
        }
    }
    return edit_distance;
}

fn find_smudged_reflection(a: &Pattern, direction: &str) -> u64 {
    use std::convert::TryFrom;

    for rowidx in 0..a.len() - 1 {
        let mut mirrorsize = 0;
        let mut mirror_edit_distance = 0;
        let mut x: u64;
        while mirrorsize <= rowidx && rowidx + mirrorsize + 1 < a.len() {
            x = row_edit_distance(&a[rowidx - mirrorsize], &a[rowidx + mirrorsize + 1]);
            if mirror_edit_distance + x > 1 {
                break;
            }
            mirror_edit_distance += x;
            mirrorsize += 1;
        }

        if mirrorsize == 0 || mirror_edit_distance != 1 {
            continue;
        }

        // It only counts if it goes up to an edge.
        if mirrorsize > rowidx && mirrorsize - rowidx == 1 || rowidx + mirrorsize + 1 == a.len() {
            //println!("found smudged reflection at {} of size {}", rowidx, mirrorsize);
            if direction == "horizontal" {
                return 100 * (u64::try_from(rowidx).unwrap() + 1);
            } else {
                return u64::try_from(rowidx).unwrap() + 1;
            }
        }
    }
    return 0;
}

fn find_smudged_horizontal_reflection(a: &Pattern) -> u64 {
    let val =  find_smudged_reflection(a, "horizontal");
    if val > 0 {
        println!("found smudged horizontal reflection, value: {}", val);
    }
    return val;
}

fn find_smudged_vertical_reflection(a: &Pattern) -> u64 {
    let cols = rotate_pattern(a);
    let val = find_smudged_reflection(&cols, "vertical");
    if val > 0 {
        println!("found smudged vertical reflection, value: {}", val);
    }
    return val;
}

fn do_pattern(a: &Pattern) -> u64 {
    //let mut val = find_horizontal_reflection(a);
    //val += find_vertical_reflection(a);
    let mut val = find_smudged_horizontal_reflection(a);
    val += find_smudged_vertical_reflection(a);
    return val;
}

fn main() {
    use std::fs::File;
    use std::io::{prelude::*, BufReader};

    let f = File::open("input.txt").unwrap();
    let reader = BufReader::new(f);

    let mut aidx = 0;
    let mut a: Pattern = vec![];
    let mut totalvalue: u64 = 0;

    for line in reader.lines() {
        let s = line.unwrap();
        if s == "" {
            println!("pattern {}", aidx);
            totalvalue += do_pattern(&a);
            a = vec![];
            aidx += 1;
            continue;
        }
        //println!("{}", s);
        let char_vec: Vec<char> = s.chars().collect();
        a.push(char_vec);
    }
    println!("pattern {}", aidx);
    totalvalue += do_pattern(&a);
    println!("total value: {}", totalvalue);
}
