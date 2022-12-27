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
    let mut history = Vec::new();
    let mut xreg = 1;
    history.push(xreg);
    for op in ops{
        match op{
            Op::Noop => {for _ in 0..1 {history.push(xreg);}},
            Op::Addx(x) => {for _ in 0..1 {
                                history.push(xreg);
                           }
                            xreg += x;
                            history.push(xreg);
                           },
        }
    }
    history
}

fn solve1(ops: Vec<Op>) -> i32{
    let history = execute(ops);
    let mut sum = 0;
    for idx in [20,60,100,140,180,220]{
        sum += (idx as i32)*history[idx-1];
    }
    sum
}

fn pretty_print(pixels: Vec<i32>){
    let mut screen = [['.'; 40]; 6];
    for pos in pixels{
        let x = (pos % 40) as usize;
        let y = (pos / 40) as usize;
        screen[y][x] = '#';
    }
    let strings = screen.map(|row| row.iter().collect::<String>());
    let string = strings.join("\n");
    print!("{}",string);
}

fn solve2(ops: Vec<Op>){
    let history = execute(ops);
    let pixels = (0..240).filter(|pos| {
                                    let xreg = history[(*pos as usize)];
                                    (xreg - (pos%40)).abs() <= 1
                                 })
                         .collect::<Vec<_>>();
    pretty_print(pixels);
}

pub fn solve(){
    assert_eq!(13140, solve1(parse_input("test1.txt")));
    println!("{:?}", solve1(parse_input("input1.txt")));
    solve2(parse_input("test1.txt"));
    print!("\n\n");
    solve2(parse_input("input1.txt"));
}
