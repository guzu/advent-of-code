use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(PartialEq)]
#[derive(Debug)]
enum Dir {
    Decrease,
    Increase
}

fn is_safe(report: &Vec<u32>) -> bool {
    let mut safe : bool = true;
    let dir = if report[0] > report[1] { Dir::Decrease } else { Dir::Increase };

    let mut i : usize = 0;
    let mut j : usize = 1;

    while j < report.len() {
        let l1 = report[i];
        let l2 = report[j];

        match dir {
            Dir::Decrease => {
                if l2 > l1 || l1-l2 < 1 || l1-l2 > 3 {
                    safe = false;
                }
            }
            Dir::Increase => {
                if l1 > l2 || l2-l1 < 1 || l2-l1 > 3 {
                        safe = false;
                }
            }
        }

        i += 1;
        j += 1;
    }

    safe
}


fn main() {
    let lines = read_lines("input.txt").unwrap();
    let mut res = 0u32;

    for line in lines {
        let l = line.unwrap();
        let report : Vec<u32> = l.split_whitespace().map(|s| s.parse::<u32>().unwrap()).collect();
        let mut rem : usize = 0;
        let mut safe : bool;
        let mut r = report.clone();
        loop {
            safe = is_safe(&r);
            if safe || rem >= report.len() {
                break;
            }
            r = report.clone();
            r.remove(rem);
            rem += 1;
        }

        res += if safe {1} else {0};
    }

    println!("{}", res);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
