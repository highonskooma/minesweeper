use std::vec;

pub fn run() {
    let row0 = String::from(".*.*.");
    let row1 = String::from("..*..");
    let row2 = String::from(".....");
    
    let matrix: Vec<String> = vec![row0,row1,row2];
    let point = (0,2);
    println!("point: {:?}",point);
    

    for elem in matrix.iter() {
        println!("{}",elem);
    }

    // for row in matrix.iter() {
    //     for elem in row.chars(){
    //         if elem == '*' {
    //             println!("bomb")
    //         }
    //     }
    // }

    let res = count_adjacent_bombs(&matrix, point);
    println!("adjacent: {}",res);
}

fn count_adjacent_bombs(matrix: &Vec<String>, (x,y): (usize,usize) ) -> i8 {
    let mut res = 0;

    let mut s: String = matrix[x].clone();
    if y>0 {
        let p: char = s.as_bytes()[y-1] as char;
        if p == '*' {
            res += 1;
        }
    }
    if y<s.len()-1 {
        let p: char = s.as_bytes()[y+1] as char;
        if p == '*' {
            res += 1;
        }
    }
    
    if x>0 {
        s = matrix[x-1].clone();
        let p: char = s.as_bytes()[y] as char;
        if p == '*' {
            res += 1;
        }
    }
    if x<matrix.len()-1 {
        s = matrix[x+1].clone();
        let p: char = s.as_bytes()[y] as char;
        if p == '*' {
            res += 1;
        }
    }
    
    res
}