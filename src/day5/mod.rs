use std::collections::HashMap;
use std::fs;
use std::path::Path;

type Move = (usize, usize, usize);

fn parse_input(fname: &str) -> (HashMap<usize, Vec<char>>, Vec<Move>){
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
                                                          .filter_map(|s| s.parse::<usize>().ok());
                                       (m_split.next().unwrap(), m_split.next().unwrap(), m_split.next().unwrap())
                                     }
                                 )
                            .collect::<Vec<_>>();

    //println!("{:?}", crate_map);
    //println!("{:?}", parsed_moves);
    (crate_map, parsed_moves)
}

fn get_tops(crate_map: HashMap<usize, Vec<char>>) -> String{
    (0..crate_map.keys().len())
                      .map(|x| crate_map.get(&(x+1)).unwrap().last().unwrap())
                      .copied()
                      .collect()
}

fn solve1((mut crate_map, moves) : (HashMap<usize, Vec<char>>, Vec<Move>)) -> String{
    for (n,from,to) in moves {
        for _ in 0..n{
            let from_col = crate_map.get_mut(&from).unwrap();
            let popped = from_col.pop().unwrap();
            let to_col = crate_map.get_mut(&to).unwrap();
            to_col.push(popped);
        }
    }

    get_tops(crate_map)
}

fn solve2((mut crate_map, moves) : (HashMap<usize, Vec<char>>, Vec<Move>)) -> String{
    for (n,from,to) in moves {
            let from_col = crate_map.get_mut(&from).unwrap();
            let popped = from_col.drain(from_col.len()-n..)
                                 .collect::<Vec<_>>();
            let to_col = crate_map.get_mut(&to).unwrap();
            to_col.extend(popped);
    }

    get_tops(crate_map)
}

pub fn solve(){
    assert_eq!("CMZ", solve1(parse_input("test1.txt")));
    println!("{:?}", solve1(parse_input("input1.txt")));
    assert_eq!("MCD", solve2(parse_input("test1.txt")));
    println!("{:?}", solve2(parse_input("input1.txt")));
}
