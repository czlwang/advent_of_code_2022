use std::path::Path;
use std::fs;

fn get_sum_cals(contents: String) -> Vec<i32>{
    let cals: Vec<Vec<i32>> = contents.split("\n\n")
                                        .map(|s| s.split("\n")
                                             .map(|ss| ss.to_string())
                                             .map(|sss| sss.parse::<i32>().unwrap())
                                             .collect()
                                        ).collect();
                                         
    return cals.iter()
                .map(|l| l.iter().sum())
                .collect();                        
    }

pub fn solve(){
    let root_file_path=Path::new("/home/czw/Documents/2022/advent_of_code_2022").join("src").join("day1");
    let file_path = root_file_path.join("input1.txt");
    let contents = fs::read_to_string(file_path).expect("file error");
    let mut sum_cals = get_sum_cals(contents);
    sum_cals.sort(); 
    let top1_cals: i32 = sum_cals.iter().rev().take(1).sum();  
    let top3_cals: i32 = sum_cals.iter().rev().take(3).sum();
    println!("{:?}", (top1_cals, top3_cals));
}