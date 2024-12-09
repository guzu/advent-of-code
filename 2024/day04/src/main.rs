use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

const XMAS : (char,char,char,char) = ('X', 'M', 'A', 'S');

fn check_L2R(m: &Vec<String>, x: usize, y: usize) -> u32 {
    let w = (
        m[y].chars().nth(x+0).unwrap(),
        m[y].chars().nth(x+1).unwrap(),
        m[y].chars().nth(x+2).unwrap(),
        m[y].chars().nth(x+3).unwrap());
    if w == XMAS { 1 } else { 0 }
}
fn check_R2L(m: &Vec<String>, x: usize, y: usize) -> u32 {
    let w = (
        m[y].chars().nth(x-0).unwrap(),
        m[y].chars().nth(x-1).unwrap(),
        m[y].chars().nth(x-2).unwrap(),
        m[y].chars().nth(x-3).unwrap());
    if w == XMAS { 1 } else { 0 }
}
fn check_T2B(m: &Vec<String>, x: usize, y: usize) -> u32 {
    let w = (
        m[y+0].chars().nth(x).unwrap(),
        m[y+1].chars().nth(x).unwrap(),
        m[y+2].chars().nth(x).unwrap(),
        m[y+3].chars().nth(x).unwrap());
    if w == XMAS { 1 } else { 0 }
}
fn check_B2T(m: &Vec<String>, x: usize, y: usize) -> u32 {
    let w = (
        m[y-0].chars().nth(x).unwrap(),
        m[y-1].chars().nth(x).unwrap(),
        m[y-2].chars().nth(x).unwrap(),
        m[y-3].chars().nth(x).unwrap());
    if w == XMAS { 1 } else { 0 }
}

// top to bottom and left to right
fn check_d1(m: &Vec<String>, x: usize, y: usize) -> u32 {
    let w = (
        m[y+0].chars().nth(x+0).unwrap(),
        m[y+1].chars().nth(x+1).unwrap(),
        m[y+2].chars().nth(x+2).unwrap(),
        m[y+3].chars().nth(x+3).unwrap());
    if w == XMAS { 1 } else { 0 }
}

fn check_d2(m: &Vec<String>, x: usize, y: usize) -> u32 {
    let w = (
        m[y+0].chars().nth(x-0).unwrap(),
        m[y+1].chars().nth(x-1).unwrap(),
        m[y+2].chars().nth(x-2).unwrap(),
        m[y+3].chars().nth(x-3).unwrap());
    if w == XMAS { 1 } else { 0 }
}
fn check_d3(m: &Vec<String>, x: usize, y: usize) -> u32 {
    let w = (
        m[y-0].chars().nth(x+0).unwrap(),
        m[y-1].chars().nth(x+1).unwrap(),
        m[y-2].chars().nth(x+2).unwrap(),
        m[y-3].chars().nth(x+3).unwrap());
    if w == XMAS { 1 } else { 0 }
}
fn check_d4(m: &Vec<String>, x: usize, y: usize) -> u32 {
    let w = (
        m[y-0].chars().nth(x-0).unwrap(),
        m[y-1].chars().nth(x-1).unwrap(),
        m[y-2].chars().nth(x-2).unwrap(),
        m[y-3].chars().nth(x-3).unwrap());
    if w == XMAS { 1 } else { 0 }
}

fn main() {
    let lines = read_lines("input.txt").unwrap();
    let mut coord = Vec::<(usize,usize)>::new();
    let mut matrix = Vec::<String>::new();
    let mut y : usize = 0;
    let mut res : u32 = 0;

    for line in lines {
        let l = line.unwrap();

        for (x, _) in l.match_indices('X') {
            coord.push((x, y));
        }

        matrix.push(l.clone());
        y += 1;
    }
    println!("num of X: {}", coord.len());

    let max_x = matrix[0].len();
    let max_y = y;
    for (x,y) in coord {
        if x < max_x - 3 { res += check_L2R(&matrix, x, y); }
        if x > 2         { res += check_R2L(&matrix, x, y); }
        if y < max_y - 3 { res += check_T2B(&matrix, x, y); }
        if y > 2         { res += check_B2T(&matrix, x, y); }

        if x < max_x - 3 && y < max_y - 3 { res += check_d1(&matrix, x, y); }
        if x > 2         && y < max_y - 3 { res += check_d2(&matrix, x, y); }
        if x < max_x - 3 && y > 2         { res += check_d3(&matrix, x, y); }
        if x > 2         && y > 2         { res += check_d4(&matrix, x, y); }

    }

    println!("result: {}", res);

}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
