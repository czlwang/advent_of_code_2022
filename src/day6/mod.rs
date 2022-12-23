use std::fs;
use std::path::Path;
use std::collections::HashSet;

pub fn parse_input(fname: &str) -> String{
    let root_path = Path::new("/home/czw/Documents/2022/advent_of_code_2022/src/day6");
    let fpath = root_path.join(fname);
    fs::read_to_string(fpath).expect("couldn't read file")
}

pub fn solve_brute_force(input: &str, window_size: usize) -> usize{
    input.chars()
         .collect::<Vec<_>>()
         .windows(window_size)
         .enumerate()
         .find(|(_i,x)|{ 
                let set: HashSet<_> = x.iter().cloned().collect();
                set.len()==window_size
         })
        .expect("couldn't find marker")
        .0
        + window_size
}

pub fn solve(){
    assert_eq!(5,solve_brute_force(&parse_input("test1.txt"), 4));
    assert_eq!(6,solve_brute_force(&parse_input("test2.txt"), 4));
    println!("{:?}",solve_brute_force(&parse_input("input1.txt"), 4));
    println!("{:?}",solve_brute_force(&parse_input("input1.txt"), 14));
}
