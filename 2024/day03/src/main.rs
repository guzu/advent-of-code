use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use regex::Regex;

fn main() {
    let lines = read_lines("input.txt").unwrap();
    let mut res : u64 = 0;

    let re = Regex::new(r"mul\(([0-9]+),([0-9]+)\)").unwrap();

    for line in lines {
        let l = line.unwrap();
        
        for m in re.captures_iter(&l) {
            let (_,[v1,v2]) = m.extract();
            res += v1.parse::<u64>().unwrap() * v2.parse::<u64>().unwrap();
        }
    }

    // part 1 : 183788984
    println!("result: {}", res);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
