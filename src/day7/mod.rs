use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::Path;

#[derive(Debug)]
enum Cmd {
    Ls,
    Cd(String),
    File {size: i32, name: String},
    DirName(String)
}

#[derive(Debug)]
struct Dir {
    files: HashMap<String, i32>,
    sub_dirs: HashSet<String>,
}

impl Dir{
    fn new() -> Self{
        Self{files: HashMap::new(), sub_dirs: HashSet::new()}
    }

    fn insert_file(&mut self, fname: &str, fsize: i32){
       self.files.insert(fname.to_string(), fsize); 
    }

    fn insert_dir(&mut self, fname: &str){
       self.sub_dirs.insert(fname.to_string()); 
    }
}

fn parse_line(line: &str) -> Cmd{
    if line.starts_with("$ cd"){
        let new_dir_name = line.split(' ').collect::<Vec<_>>()[2];
        Cmd::Cd(new_dir_name.to_string())
    }else if line.starts_with("$ ls"){
        Cmd::Ls
    }else if line.starts_with("dir"){
        let dir_name = line.split(' ').collect::<Vec<_>>()[1];
        Cmd::DirName(dir_name.to_string())
    }else{
        let line_elems = line.split(' ').collect::<Vec<_>>();
        Cmd::File { size: line_elems[0].parse().unwrap(), name: line_elems[1].to_string()}
    }
}

fn parse_cmds(cmds: Vec<Cmd>) -> HashMap<String, Dir>{
    let mut dir_stack: Vec<String> = Vec::new(); 
    let mut dirs: HashMap<String, Dir> = HashMap::new();

    for cmd in cmds{
        if !dir_stack.is_empty(){
            let full_name = dir_stack.join("/");
            if !dirs.contains_key(&full_name){
                dirs.insert(full_name.to_string(), Dir::new());
            }
        }

        match cmd {
            Cmd::Ls => (),
            Cmd::File{size, name} => {
                let full_name = dir_stack.join("/");
                let curr_dir = dirs.get_mut(&full_name).expect("couldn't find dir");
                curr_dir.insert_file(&name, size);
            },
            Cmd::DirName(name) => {
                let mut full_name = dir_stack.join("/");
                let curr_dir = dirs.get_mut(&full_name).expect("couldn't find dir");
                full_name = full_name + "/" + &name;
                curr_dir.insert_dir(&full_name);
            },
            Cmd::Cd(name) => {
                if name==".." {
                    dir_stack.pop();
                }else{
                    dir_stack.push(name);
                }
            },
        }
    }

    dirs
}

fn parse_input(fname: &str) -> Vec<Cmd>{
    let root_path = Path::new("/home/czw/Documents/2022/advent_of_code_2022/src/day7");
    let fpath = root_path.join(fname);
    let string = fs::read_to_string(fpath).expect("couldn't find file");
    let cmds = string.trim()
                     .split('\n')
                     .map(parse_line)
                     .collect::<Vec<_>>();

    cmds
}

fn get_dir_size(dir_name: &str, dir_map: &HashMap<String, Dir>, dir_sizes: &mut HashMap<String, i32>) -> i32{
    if dir_sizes.contains_key(dir_name){
        return *dir_sizes.get(dir_name).unwrap();
    }
    
    //println!("{:?}", dir_name);
    let curr_dir = dir_map.get(dir_name).expect("no such dir");
    let sub_dirs = &curr_dir.sub_dirs;
    let mut sum = curr_dir.files.values().sum::<i32>();
    for sub_dir in sub_dirs{
        sum += get_dir_size(sub_dir, dir_map, dir_sizes);
    }
    dir_sizes.insert(dir_name.to_string(), sum);
    sum
}

fn solve1(cmds: Vec<Cmd>) -> i32{
    let dir_map = parse_cmds(cmds);
    let mut dir_sizes = HashMap::new();
    get_dir_size("/", &dir_map, &mut dir_sizes);
    dir_sizes.values()
             .filter(|x| **x < 100000)
             .sum()
}

pub fn solve(){
    assert_eq!(95437, solve1(parse_input("test1.txt")));
    println!("{:?}", solve1(parse_input("input1.txt")));
}
