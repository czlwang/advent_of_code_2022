use std::io::BufReader;
use std::io::BufRead;
use std::path::Path;
use std::fs;

pub fn hello(){
    println!("hello");
    let root_file_path=Path::new("/home/czw/Documents/2022/advent_of_code_2022").join("src").join("day1");
    // let file_path = root_file_path.join("test1.txt"); 
    let file_path = root_file_path.join("input1.txt"); 
    println!("{}", file_path.display());
    let contents = fs::read_to_string(file_path).expect("file error");
    let cals: Vec<Vec<i32>> = contents.split("\n\n")
                                        .map(|s| s.split("\n")
                                             .map(|ss| ss.to_string())
                                             .map(|sss| sss.parse::<i32>().unwrap())
                                             .collect()
                                        ).collect();
                                         
    let mut sum_cals: Vec<i32> = cals.iter()
                                 .map(|l| l.iter().sum())
                                 .collect();
                                 
    // let max_cals = sum_cals.iter().max().expect("no max");   
    sum_cals.sort(); 
    let top1_cals: i32 = sum_cals.iter().rev().take(1).sum();  
    let top3_cals: i32 = sum_cals.iter().rev().take(3).sum();                         
    // let file = fs::File::open(file_path).expect("Should have been able to read the file");
    // let buf = BufReader::new(file);
    // let lines: Vec<String> = buf.lines().map(|l| l.expect("error parsing")).collect();
    // println!("{}",lines);
    println!("{:?}", (top1_cals, top3_cals));
}
