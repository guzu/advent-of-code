use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
 
fn main() {
    let lines = read_lines("input.txt").unwrap();
    let mut coord = Vec::<(usize,usize)>::new();
    let mut matrix = Vec::<String>::new();
    let mut y : usize = 0;
    let mut res : u32 = 0;

    for line in lines {
        let l = line.unwrap();

        for (x, _) in l.match_indices('A') {
            coord.push((x, y));
        }

        matrix.push(l.clone());
        y += 1;
    }

    let max_x = matrix[0].len();
    let max_y = y;
    for (x,y) in coord {
        if x > 0 && x < max_x - 1 && y > 0 && y < max_y - 1
        {
            let l1 = matrix[y-1].chars().nth(x-1).unwrap();
            let l2 = matrix[y-1].chars().nth(x+1).unwrap();
            let l3 = matrix[y+1].chars().nth(x-1).unwrap();
            let l4 = matrix[y+1].chars().nth(x+1).unwrap();
            if ((l1 == 'M' && l4 == 'S') || (l1 == 'S' && l4 == 'M')) &&
               ((l2 == 'M' && l3 == 'S') || (l2 == 'S' && l3 == 'M'))
            { res += 1; }
        }
    }

    println!("result: {}", res);

}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
