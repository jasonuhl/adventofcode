use indexmap::IndexMap;

fn hash(s: &String) -> u8 {
    let mut val: u16 = 0;
    for c in s.bytes() {
        val += c as u16;
        val *= 17;
        val %= 256;
    }
    return val.try_into().unwrap();
}

fn do_part(boxes: &mut Vec<IndexMap<String, u32>>, label: &String, sep: &String, focallength: &String) {
    let boxidx: usize = usize::try_from(hash(label)).unwrap();
    println!("{} {} {}: box {}", label, sep, focallength, boxidx);
    if sep == "-" {
        if boxes[boxidx].get(label).is_some() {
            println!("removing {}", label);
            boxes[boxidx].shift_remove(label);
        }
    } else if sep == "=" {
        if boxes[boxidx].get(label).is_some() {
            println!("replacing {}", label);
            boxes[boxidx][label] = focallength.parse().unwrap();
            let mut v = boxes[boxidx].get_mut(label).unwrap();
            *v = focallength.parse().unwrap();
        } else {
            println!("inserting {}", label);
            boxes[boxidx].insert(label.clone(), focallength.parse().unwrap());
        }
    }
}

fn get_total(boxes: &Vec<IndexMap<String, u32>>) {
    let mut total: u64 = 0;
    for (boxidx, lensbox) in boxes.iter().enumerate() {
        for (lensidx, lens) in lensbox.iter().enumerate() {
            let k = lens.0;
            let v = lens.1;
            println!("boxidx {} lensidx {} lens {:?} {} {}", boxidx, lensidx, lens, k, v);
            total += (u64::try_from(boxidx).unwrap() + 1) * (u64::try_from(lensidx).unwrap() + 1) * u64::try_from(*v).unwrap();
        }
    }
    println!("total: {}", total);
}

fn do_line(line: String) {
    use regex::Regex;

    let mut boxes: Vec<IndexMap<String, u32>> = vec![];
    for _ in 0..256 {
        boxes.push(IndexMap::new());
    }

    //*boxes[0].entry("foo").or_insert(5) += 1;
    //let bar = boxes[0].get("foo").unwrap();
    //println!("bar is {:?}", bar);

    let re = Regex::new(r"(.*)(=|-)(.*)").unwrap();
    let parts = line.split(",");
    //println!("Parts: {:?}", parts);
    let mut _hashtotal: u64 = 0;
    for part in parts {
        let Some(caps) = re.captures(part) else {
            panic!("no match");
        };
        do_part(&mut boxes, &caps[1].to_string(), &caps[2].to_string(), &caps[3].to_string());
    }

    get_total(&boxes);
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
