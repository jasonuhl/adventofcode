fn hash(s: &str) -> u8 {
    use std::convert::TryInto;

    let mut val: u16 = 0;
    for c in s.bytes() {
        val += c as u16;
        val *= 17;
        val %= 256;
    }
    return val.try_into().unwrap();
}

fn do_line(line: String) {
    let parts = line.split(",");
    //println!("Parts: {:?}", parts);
    let mut hashtotal: u64 = 0;
    for part in parts {
        println!("{}", part);
        hashtotal += hash(part) as u64;
    }
    println!("hashtotal: {}", hashtotal);
}

fn main() {
    use std::fs::File;
    use std::io::{prelude::*, BufReader};

    let f = File::open("input.txt").unwrap();
    let reader = BufReader::new(f);

    for line in reader.lines() {
        let s = line.unwrap();
        do_line(s);
    }
}
