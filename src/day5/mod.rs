use std::collections::HashMap;
use std::fs;
use std::path::Path;

type Move = (i32, i32, i32);

fn parse_input(fname: &str){
    let root_path = Path::new("/home/czw/Documents/2022/advent_of_code_2022/src/day5");
    let fpath = root_path.join(fname);
    let string = fs::read_to_string(fpath).expect("no file");
    let mut crates_moves = string.split("\n\n");
    let (crates, moves) = (crates_moves.next().unwrap(), crates_moves.next().unwrap());

    let mut crate_map: HashMap<usize, Vec<char>> = HashMap::new();
    for line in crates.split('\n').rev().skip(1){
        let chunks = line.chars()
                        .collect::<Vec<char>>()
                        .chunks(4)
                        .map(|x| x.to_owned())
                        .collect::<Vec<_>>();
        for (i,c) in chunks.iter().enumerate(){
            if c[0] == '[' { 
                crate_map.entry(i+1).or_insert_with(Vec::new);
                crate_map.get_mut(&(i+1)).unwrap().push(c[1]);
            }
        }
    }

    let parsed_moves = moves.trim()
                            .split('\n')
                            .map(|m| 
                                 {
                                   let mut m_split = m.split(' ')
                                                      .filter_map(|s| s.parse::<i32>().ok());
                                   (m_split.next().unwrap(), m_split.next().unwrap(), m_split.next().unwrap())
                                 }
                                 )
                            .collect::<Vec<_>>();

    println!("{:?}", crate_map);
    println!("{:?}", parsed_moves);
}

pub fn solve(){
    println!("{:?}", parse_input("test1.txt"));
}
