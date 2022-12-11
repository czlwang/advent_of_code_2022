use std::fs;
use std::io::BufReader;
use std::io::BufRead;
use std::path::Path;

type Interval = (i32, i32);
type Pair = (Interval, Interval);

fn parse_interval(line: &str) -> Interval{
    let mut split = line.split('-');
    (split.next().unwrap().parse::<i32>().unwrap(), split.next().unwrap().parse::<i32>().unwrap())
}

fn parse_line(line: String) -> Pair{
    let mut split = line.split(',');
    (parse_interval(split.next().unwrap()), parse_interval(split.next().unwrap()))
}

fn parse_input(fname: &str) -> Vec<Pair>{
    let root_path = Path::new("/home/czw/Documents/2022/advent_of_code_2022/src/day4/");    
    let fpath = root_path.join(fname);
    let file = fs::File::open(fpath).expect("couldn't find file");
    BufReader::new(file).lines()
                        .map(|l| l.unwrap())
                        .map(parse_line)
                        .collect::<Vec<_>>()
}

fn solve1(pairs: Vec<Pair>) -> usize{
    pairs.iter()
         .filter(|(p1,p2)| 
              ((p1.0 >= p2.0 && p1.1 <= p2.1) || (p2.0 >= p1.0 && p2.1 <= p1.1)))
         .count()
}

pub fn solve(){
    println!("Hello");
    assert_eq!(2, solve1(parse_input("test1.txt")));
    println!("{:?}", solve1(parse_input("input1.txt")));
}
