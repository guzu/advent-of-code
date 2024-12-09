use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use regex::Regex;

fn main() {
    let lines = read_lines("input.txt").unwrap();
    let mut res : u64 = 0;
    let mut enabled = 1;
    let re = Regex::new(r"do\(\)|don't\(\)|mul\([0-9]+,[0-9]+\)").unwrap();
    let re_mul = Regex::new(r"mul\(([0-9]+),([0-9]+)\)").unwrap();

    for line in lines {
        let l = line.unwrap();
        
        for mat in re.find_iter(&l) {
            if mat.as_str() == "do()"       { enabled = 1;  }
            else if mat.as_str() == "don't()" { enabled = 0; }
            else {
                let (_,[v1,v2]) = re_mul.captures_iter(mat.as_str()).nth(0).unwrap().extract();
                res += enabled * v1.parse::<u64>().unwrap() * v2.parse::<u64>().unwrap();
            }
        }
    }

    // part 1 : 183788984
    // part 2 : 62098619
    println!("result: {}", res);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
