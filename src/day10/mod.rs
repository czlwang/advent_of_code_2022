use std::fs;
use std::path::Path;

#[derive(Debug)]
enum Op{
    Noop,
    Addx(i32)
}

fn parse_input(fname: &str) -> Vec<Op>{
    let root_path = Path::new("/home/czw/Documents/2022/advent_of_code_2022/src/day10");
    let fpath = root_path.join(fname);
    let strings = fs::read_to_string(fpath).expect("couldn't find file");
    strings.trim()
           .split('\n')
           .map(|line| {
               if line.starts_with("noop"){
                   Op::Noop
               }else if line.starts_with("addx"){
                   let ll = line.split(' ').collect::<Vec<_>>();
                   let num = ll[1].parse::<i32>().expect("couldn't parse");
                   Op::Addx(num)
               }else{
                   panic!("couldn't parse");
               }
           })
           .collect::<Vec<_>>()
}

fn execute(ops: Vec<Op>) -> Vec<i32>{
    println!("{:?}", &ops[..21]);
    let mut history = Vec::new();
    let max_delay = 2;
    let mut xreg = 1;
    for i in 0..(ops.len()+max_delay){
        for lag in 0..max_delay{
            if (0 <= (i as i32) - (lag as i32)) && (i-lag < ops.len()){
                let op = &ops[i-lag];
                println!("{:?}", (op, xreg));
                match op {
                    Op::Noop => {if lag==0 {}},
                    Op::Addx(x) => {if lag==1 {xreg += x}},
                }
            }
        }
        history.push(xreg);
    }
    println!("{:?}", history);
    history
}

fn solve1(ops: Vec<Op>) -> i32{
    //ops.into_iter()
    //   .map(|op| match op { Op::Addx(x) => x, Op::Noop => 0 })
    //   .sum()
    let history = execute(ops);
    println!("{:?}", (history[19], history[20], history[21]));
    1
    //history[20]*history[60]*history[100]*history[140]*history[180]*history[220]
}

pub fn solve(){
    assert_eq!(13140, solve1(parse_input("test1.txt")));
    //assert_eq!(13140, solve1(parse_input("test2.txt")));
}
