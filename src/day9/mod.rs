use std::fs;
use std::collections::HashSet;
use std::path::Path;

type Pos = (i32, i32);

fn parse_input(fname: &str) -> Vec<(String, i32)>{
    let root_path = Path::new("/home/czw/Documents/2022/advent_of_code_2022/src/day9/");
    let fpath = root_path.join(fname);
    let strings = fs::read_to_string(fpath).expect("no file");
    strings.trim()
           .split('\n')
           .map(|x|{
                let mut tupled = x.split(' ');
                //println!("{:?}", tupled.next().unwrap());
                //println!("{:?}", tupled.next().unwrap());
                (tupled.next().expect("couldn't split 1").to_string(), 
                 tupled.next()
                       .expect("couldn't split 2")
                       .parse::<i32>()
                       .expect("couldn't parse"))
           })
          .collect::<Vec<_>>()
}

fn move_ht(hmove: &str, head: Pos, tail: Pos) -> (Pos, Pos){
    let (hx, hy) = head;
    let new_head = match hmove {
        "R" => (hx+1, hy),
        "L" => (hx-1, hy),
        "U" => (hx, hy+1),
        "D" => (hx, hy-1),
        _ => panic!("not supposed to get here")
    };

    let (tx, ty) = tail;
    let (hx, hy) = new_head;
    let (dx, dy) = (hx-tx, hy-ty);
    let mut new_tail = (tx, ty);

    if dx.abs() > 1 || dy.abs() > 1 {
        new_tail = (dx.signum() + tx, dy.signum() + ty);
    }

    (new_head, new_tail)
}

fn solve1(moves: Vec<(String, i32)>) -> usize{
     let mut tail = (0,0);
     let mut head = (0,0); 
     let mut visited: HashSet<Pos> = HashSet::new();
     visited.insert((0,0));
     for (m, steps) in moves{
         //println!("{:?}",(&m,steps));
         for _ in 0..steps{
             (head, tail) = move_ht(&m, head, tail);
             //println!("{:?}", (head, tail));
             visited.insert(tail);
         }
    } 
    visited.len()
}

pub fn solve(){
    assert_eq!(13, solve1(parse_input("test1.txt")));
    println!("{:?}", solve1(parse_input("input1.txt")));
}
