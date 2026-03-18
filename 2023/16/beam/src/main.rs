#[derive(Debug)]
#[derive(Copy, Clone)]
struct Beam {
    y: usize,
    x: usize,
    direction: char,
}

//fn pretty_print(a: &Vec<Vec<char>>) {
//    for row in a {
//        for col in row {
//            print!("{}", col);
//        }
//        println!("");
//    }
//}
//
fn pretty_print_e(e: &Vec<Vec<u8>>) {
    for row in e {
        for c in row {
            if *c > 0 {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!("");
    }
}

fn advance_beam(beam: &mut Beam, a: &Vec<Vec<char>>) -> bool {
    if beam.direction == 'R' {
        if beam.x == a[0].len() - 1 {
            return false;
        }
        beam.x += 1;
    } else if beam.direction == 'L' {
        if beam.x == 0 {
            return false;
        }
        beam.x -= 1;
    } else if beam.direction == 'D' {
        if beam.y == a.len() - 1 {
            return false;
        }
        beam.y += 1;
    } else if beam.direction == 'U' {
        if beam.y == 0 {
            return false;
        }
        beam.y -= 1;
    }
    return true;
}

fn do_beam(beams: &mut Vec<Beam>, beamidx: usize, a: &Vec<Vec<char>>, e: &mut Vec<Vec<u8>>) -> bool {
    let mut beam = &mut beams[beamidx];
    //println!("Doing {:?}", beam);

    let energized_bit: u8 = match beam.direction {
        'R' => 1,
        'L' => 2,
        'D' => 4,
        'U' => 8,
        _ => panic!("unknown direction"),
    };
    if e[beam.y][beam.x] & energized_bit == 1 {
        //println!("in a loop");
        return false;
    } else {
        e[beam.y][beam.x] |= energized_bit;
    }
    //e[beam.y][beam.x] = '#';

    if a[beam.y][beam.x] == '.' {
        return advance_beam(&mut beam, a);
    } else if a[beam.y][beam.x] == '|' {
        if beam.direction == 'D' || beam.direction == 'U' {
            return advance_beam(&mut beam, a);
        } else {
            //println!("splitting");
            beam.direction = 'D';
            let mut newbeam = beam.clone();
            newbeam.direction = 'U';
            beams.push(newbeam);
        }
    } else if a[beam.y][beam.x] == '-' {
        if beam.direction == 'R' || beam.direction == 'L' {
            return advance_beam(&mut beam, a);
        } else {
            //println!("splitting");
            beam.direction = 'R';
            let mut newbeam = beam.clone();
            newbeam.direction = 'L';
            beams.push(newbeam);
        }
    } else if a[beam.y][beam.x] == '/' {
        if beam.direction == 'R' {
            beam.direction = 'U';
        } else if beam.direction == 'L' {
            beam.direction = 'D';
        } else if beam.direction == 'D' {
            beam.direction = 'L';
        } else if beam.direction == 'U' {
            beam.direction = 'R';
        }
        return advance_beam(&mut beam, a);
    } else if a[beam.y][beam.x] == '\\' {
        if beam.direction == 'R' {
            beam.direction = 'D';
        } else if beam.direction == 'L' {
            beam.direction = 'U';
        } else if beam.direction == 'D' {
            beam.direction = 'R';
        } else if beam.direction == 'U' {
            beam.direction = 'L';
        }
        return advance_beam(&mut beam, a);
    } else {
        panic!("unknown char in a");
    }

    return true;
}

fn get_total(e: &Vec<Vec<u8>>) -> u64 {
    let mut total: u64 = 0;
    for row in e {
        for c in row {
            if *c > 0 {
                total += 1;
            }
        }
    }
    return total;
}

fn do_initial_beam(a: &Vec<Vec<char>>, initial_y: usize, initial_x: usize, initial_direction: char) {
    let mut beams: Vec<Beam> = vec![];
    beams.push(Beam {x: initial_x, y: initial_y, direction: initial_direction});

    let mut e: Vec<Vec<u8>> = Vec::with_capacity(a.len());
    for _ in 0..a.len() {
        e.push(vec![0; a[0].len()])
    }

    while beams.len() > 0 {
        let mut to_remove: Vec<usize> = vec![];
        for beamidx in 0..beams.len() {
            //println!("doing beamidx {}", beamidx);
            if !do_beam(&mut beams, beamidx, &a, &mut e) {
                //println!("need to terminate this beam: {:?}", beams[beamidx]);
                to_remove.push(beamidx);
            }
        }
        //pretty_print_e(&e);
        //println!("Total: {}", get_total(&e));
        let mut fudge: usize = 0;
        for idx in to_remove {
            beams.remove(idx - fudge);
            fudge += 1;
        }
    }
    //pretty_print_e(&e);
    println!("{} Total for initial x={} y={} dir={}", get_total(&e), initial_x, initial_y, initial_direction);
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

    // top
    for x in 0..a[0].len() {
        do_initial_beam(&a, 0, x, 'D');
    }

    // bottom
    for x in 0..a[0].len() {
        do_initial_beam(&a, a.len() - 1, x, 'U');
    }

    // left
    for y in 0..a.len() {
        do_initial_beam(&a, y, 0, 'R');
    }

    // right
    for y in 0..a.len() {
        do_initial_beam(&a, y, a[0].len() - 1, 'L');
    }
}
