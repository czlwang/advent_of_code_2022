use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;
use std::io::BufReader;
use std::io::BufRead;
use std::path::Path;

fn read_lines(fname: &str) -> Vec<String>{
    let root_path = Path::new("/home/czw/Documents/2022/advent_of_code_2022/src/day3/");
    let fpath = root_path.join(fname);
    let file = fs::File::open(fpath).expect("no such file");
    BufReader::new(file).lines()
                        .map(|l| l.expect("couldn't parse line"))
                        .collect()
}

fn line2sets(line: &str) -> (HashSet<char>, HashSet<char>){
    let length = line.chars().count();
    assert!(length%2 == 0);
    let half = length/2;
    (line.chars().take(half).collect(), line.chars().rev().take(half).collect())
}

fn solve1(lines: Vec<String>, value_map: &HashMap<char, usize>) -> usize{
    lines.iter()
         .map(|l| line2sets(l))
         .map(|(x,y)| *x.intersection(&y).next().expect("no intersection"))
         .map(|c| *value_map.get(&c).expect("not in map"))
         .sum()
}

fn find_common(group: &[HashSet<char>]) -> char{
    let mut intersection = group[0].clone();
    for g in group{
       intersection.retain(|c| g.contains(c)); 
    }
    *intersection.iter().next().unwrap()
}

fn solve2(lines: Vec<String>, value_map: &HashMap<char, usize>) -> usize{
    lines.iter()
         .map(|l| l.chars().collect::<HashSet<char>>())
         .collect::<Vec<HashSet<char>>>()
         .chunks(3)
         .map(find_common)
         .map(|c| *value_map.get(&c).expect("not in map"))
         .sum()
}

pub fn solve(){
    let mut value_map: HashMap<char, usize> = HashMap::new();
    for (i, c) in "abcdefghijklmnopqrstuvwxyz".chars().enumerate(){
        value_map.insert(c,i+1);
    }
    for (i, c) in "ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().enumerate(){
        value_map.insert(c,i+27);
    }
 
    let test1_result = solve1(read_lines("test1.txt"), &value_map);
    assert!(test1_result==157);
    println!("{:?}",solve1(read_lines("input1.txt"), &value_map));

    let test2_result = solve2(read_lines("test1.txt"), &value_map);
    assert!(test2_result==70);
    println!("{:?}",solve2(read_lines("input1.txt"), &value_map));
}
