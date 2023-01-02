use std::fs;
use std::path::Path;
use std::collections::{HashMap, HashSet};

type Grid = Vec<Vec<i32>>;
type Pos = (i32, i32);

fn parse_input(fname: &str) -> (Grid, Pos, Pos){
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
                                                    'S' => {start=(row as i32, col as i32); 
                                                            *char2int.get(&'a').expect("no char")},
                                                    'E' => {end=(row as i32, col as i32);
                                                            *char2int.get(&'z').expect("no char")},
                                                     c  => *char2int.get(&c).expect("no char"),
                                            }})
                                     .collect::<Vec<_>>())
                     .collect::<Vec<_>>();
    (grid, start, end)
}

fn get_pos(grid: &Grid, pos: Pos) -> i32{
    grid[(pos.0 as usize)][(pos.1 as usize)]
}

fn get_nbrs(grid: &Grid, pos: Pos, backwards: bool) -> Vec<Pos>{
    let rows = grid.len() as i32;
    let cols = grid.get(0).unwrap().len() as i32;
    let elevation = get_pos(grid, pos);

    let nbrs = vec![(0,1), (0,-1), (1,0), (-1,0)]
               .iter()
               .map(|nbr_delta| (pos.0 + nbr_delta.0, pos.1 + nbr_delta.1))
               .filter(|nbr_pos| nbr_pos.0 < rows && nbr_pos.0 >= 0 
                              && nbr_pos.1 < cols && nbr_pos.1 >= 0)
               .collect::<Vec<_>>();

    if !backwards {
        nbrs.iter()
            .filter(|nbr_pos| get_pos(grid, **nbr_pos) <= elevation + 1)
            .copied()
            .collect::<Vec<_>>()
    }else{
        nbrs.iter()
            .filter(|nbr_pos| get_pos(grid, **nbr_pos) >= elevation - 1)
            .copied()
            .collect::<Vec<_>>()
    }
}

fn solve1((grid, start, end): (Grid, Pos, Pos)) -> i32{
    let mut queue: Vec<(Pos, i32)> = Vec::new();
    queue.push((start, 0));
    let mut visited: HashSet<Pos> = HashSet::new();
    while !queue.is_empty() {
        let (next, length) = queue.remove(0);
        if visited.contains(&next){
            continue;
        }
        visited.insert(next);
        let nbrs = get_nbrs(&grid, next, false)
                            .iter()
                            .filter(|nbr| !visited.contains(nbr))
                            .map(|nbr| ((*nbr), length+1))
                            .collect::<Vec<_>>();
        for (nbr, i) in &nbrs{
            if *nbr==end{
                return *i;
            }
        }
        queue.extend(nbrs);
    }
    -1
}

fn solve2((grid, _, end): (Grid, Pos, Pos)) -> i32{
    let mut queue: Vec<(Pos, i32)> = Vec::new();
    queue.push((end, 0));
    let mut visited: HashSet<Pos> = HashSet::new();
    let mut min_dist = (grid.len()*grid[0].len()) as i32;
    while !queue.is_empty() {
        let (next, length) = queue.remove(0);
        if visited.contains(&next){
            continue;
        }
        visited.insert(next);
        let nbrs = get_nbrs(&grid, next, true)
                            .iter()
                            .filter(|nbr| !visited.contains(nbr))
                            .map(|nbr| ((*nbr), length+1))
                            .collect::<Vec<_>>();
        for (nbr, i) in &nbrs{
            if get_pos(&grid, *nbr) == 0 {
                min_dist = std::cmp::min(*i, min_dist);
            }
        }
        queue.extend(nbrs);
    }
    min_dist
}

pub fn solve(){
    assert_eq!(31, solve1(parse_input("test1.txt")));
    println!("{:?}", solve1(parse_input("input1.txt")));
    assert_eq!(29, solve2(parse_input("test1.txt")));
    println!("{:?}", solve2(parse_input("input1.txt")));
}


