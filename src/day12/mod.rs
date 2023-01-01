use std::fs;
use std::path::Path;
use std::collections::HashMap;

fn parse_input(fname: &str){
    let root_path = Path::new("/home/czw/Documents/2022/advent_of_code_2022/src/day12/");
    let fpath = root_path.join(fname);
    let string = fs::read_to_string(fpath).expect("no file");
    let mut char2int: HashMap<char, i32> = HashMap::new();
    let alphabet = "abcdefghijklmnopqrstuvwxyz";
    for (i,c) in alphabet.chars().enumerate(){
        char2int.insert(c,i as i32);
    }
    let mut start = (0,0);
    let mut end = (0,0);
    let grid = string.trim()
                     .split('\n')
                     .enumerate()
                     .map(|(row, line)| line.chars()
                                            .enumerate()
                                            .map(|(col, c)|
                                                 {match c {
                                                    'S' => {start=(row, col); 
                                                            *char2int.get(&'a').expect("no char")},
                                                    'E' => {end=(row,col);
                                                            *char2int.get(&'z').expect("no char")},
                                                     c  => *char2int.get(&c).expect("no char"),
                                            }})
                                     .collect::<Vec<_>>())
                     .collect::<Vec<_>>();
    println!("{:?}", (grid, start, end));
}

pub fn solve(){
    parse_input("test1.txt");
    println!("hello");
}


