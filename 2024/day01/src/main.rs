use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let lines = read_lines("input.txt").unwrap();
    let mut c1: Vec<u32> = Vec::<u32>::new();
    let mut c2: Vec<u32> = Vec::<u32>::new();

    for line in lines {
        let l = line.unwrap();
        let a : Vec<u32> = l.split_whitespace().map(|s| s.parse::<u32>().unwrap()).collect();
        c1.push(a[0]);
        c2.push(a[1]);
    }
    c1.sort();
    c2.sort();

    // Part 1
    let mut res : u32 = 0;
    for (i, j) in c1.iter().zip(c2.iter()) {
        res += if i > j { i-j } else { j-i };
    }
    println!("part1: {}", res);

    // Part 2
    res = 0;
    for i in &c1 {
        let c = c2.iter().fold(0, |tot,n| if n == i {tot + 1} else { tot } );
        res += i * c;
    }
    println!("part2: {}", res);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
