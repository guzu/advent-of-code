use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(PartialEq)]
#[derive(Debug)]
enum Dir {
    Decrease,
    Increase
}

fn main() {
    let lines = read_lines("input.txt").unwrap();
    let mut res = 0u32;

    for line in lines {
        let l = line.unwrap();
        let report : Vec<u32> = l.split_whitespace().map(|s| s.parse::<u32>().unwrap()).collect();
        let mut safe : bool = true;
        let dir = if report[0] > report[1] { Dir::Decrease } else { Dir::Increase };

        for (l1,l2) in report.iter().zip(report.iter().skip(1)) {
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
