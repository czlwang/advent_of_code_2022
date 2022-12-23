use std::fs;
use std::path::Path;

pub fn parse_input(fname: &str) -> String{
    let root_path = Path::new("/home/czw/Documents/2022/advent_of_code_2022/src/day6");
    let fpath = root_path.join(fname);
    fs::read_to_string(fpath).expect("couldn't read file")
}

pub fn solve(){
    println!("{:?}",parse_input("test1.txt"));
}
