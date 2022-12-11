use std::collections::HashMap;
use std::fs;
use std::io::BufReader;
use std::io::BufRead;
use std::path::Path;

fn parse_input(fname: &str){
    let root_path = Path::new("/home/czw/Documents/2022/advent_of_code_2022/src/day5");
    let fpath = root_path.join(fname);
    let string = fs::read_to_string(fpath).expect("no file");
    let mut crates_instrs = string.split("\n\n");
    let (crates, instrs) = (crates_instrs.next().unwrap(), crates_instrs.next().unwrap());
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
        println!();
    }
    println!("{:?}", crate_map);
    println!("hello");
    println!("{:?}", instrs);
    //let file = fs::File::open(fpath).expect("no file");
    //let lines = BufReader::new(file).read_to_string();
                                    //.map(|l| l.unwrap());

}
pub fn solve(){
    println!("{:?}", parse_input("test1.txt"));
}
