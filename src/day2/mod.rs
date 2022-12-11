use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs;
use std::io::BufReader;
use std::io::BufRead;
use std::path::Path;
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Choice {
    Rock,
    Paper,
    Scissors
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum End {
    Lose,
    Win,
    Draw
}

fn result2choice(e_choice: &Choice, end: &End) -> Choice {
    match e_choice{
        Choice::Rock => match end {
            End::Win => Choice::Paper,
            End::Lose => Choice::Scissors,
            End::Draw => Choice::Rock,
        }
        Choice::Paper => match end{
            End::Win => Choice::Scissors,
            End::Lose => Choice::Rock,
            End::Draw => Choice::Paper,
        }
        Choice::Scissors => match end{
            End::Win => Choice::Rock,
            End::Lose => Choice::Paper,
            End::Draw => Choice::Scissors,
        }
    }
}

fn cmp_choices(c1: &Choice, c2: &Choice) -> Ordering {
    match c1{
        Choice::Rock => match c2 {
            Choice::Rock => Ordering::Equal,
            Choice::Paper => Ordering::Less,
            Choice::Scissors => Ordering::Greater,
        }
        Choice::Paper => match c2{
            Choice::Rock => Ordering::Greater,
            Choice::Paper => Ordering::Equal,
            Choice::Scissors => Ordering::Less
        }
        Choice::Scissors => match c2{
            Choice::Rock => Ordering::Less,
            Choice::Paper => Ordering::Greater,
            Choice::Scissors => Ordering::Equal
        }
    }
}

fn parse_line(line: &str) -> (String, String){
    let mut words = line.split(' ');
    (words.next().unwrap().to_string(), words.next().unwrap().to_string())
}

fn parse_lines(fname: &str) -> Vec<String>{
    let root_path = Path::new("/home/czw/Documents/2022/advent_of_code_2022/src/day2/");
    let fpath = root_path.join(fname);

    let file = fs::File::open(fpath).expect("no such file");
    BufReader::new(file).lines()
                        .map(|l| l.expect("couldn't parse line"))
                        .collect()
}

fn parse_input1(fname: &str) -> Vec<(Choice, Choice)>{
    let lines = parse_lines(fname);
                                                                  
    let player2choice = HashMap::from([("X".to_string(), Choice::Rock),
                                       ("Y".to_string(), Choice::Paper),
                                       ("Z".to_string(), Choice::Scissors)]);
    let elf2choice = HashMap::from([("A".to_string(), Choice::Rock),
                                    ("B".to_string(), Choice::Paper),
                                    ("C".to_string(), Choice::Scissors)]); 
    
                                                       
    lines.iter()
         .map(|l| parse_line(l))
         .map(|(e,p)| (elf2choice[&e], player2choice[&p]))
         .collect()
}

fn parse_input2(fname: &str) -> Vec<(Choice, End)>{
    let lines = parse_lines(fname);
                                                                  
    let player2choice = HashMap::from([("X".to_string(), End::Lose),
                                       ("Y".to_string(), End::Draw),
                                       ("Z".to_string(), End::Win)]);
    let elf2choice = HashMap::from([("A".to_string(), Choice::Rock),
                                    ("B".to_string(), Choice::Paper),
                                    ("C".to_string(), Choice::Scissors)]); 
     
                                                       
    lines.iter()
         .map(|l| parse_line(l))
         .map(|(e,p)| (elf2choice[&e], player2choice[&p]))
         .collect()
}

fn round_score((e_choice, p_choice):(&Choice, &Choice)) -> i64 {
    let shape2score = HashMap::from([(Choice::Rock, 1),
                                 (Choice::Paper, 2),
                                 (Choice::Scissors, 3)]);
    let shape_score = shape2score[p_choice];
    let winner_score = match cmp_choices(e_choice, p_choice) {
        Ordering::Equal => 3 ,
        Ordering::Greater => 0,
        Ordering::Less => 6
    };
    shape_score + winner_score
}

fn solve1(strat: Vec<(Choice, Choice)>) -> i64{
    strat.iter()
         .map(|(e1,c1)| round_score((e1,c1)))
         .sum()
}

fn solve2(choices_ends: Vec<(Choice,End)>) -> i64{
    let strat: Vec<(Choice, Choice)> = choices_ends.iter()
                                                   .map(|(choice, end)| (choice.clone(), result2choice(choice, end)))
                                                   .collect();
    solve1(strat)
}

pub fn solve() {
    let test1_input = parse_input1("test1.txt");
    let test1_result = solve1(test1_input);
    assert_eq!(test1_result, 15);

    let solve1_input = parse_input1("input1.txt");
    let solve1_result = solve1(solve1_input);
    println!("{:?}", solve1_result);

    let test2_input = parse_input2("test1.txt");
    let test2_result = solve2(test2_input);
    assert_eq!(test2_result, 12);
    
    let solve2_input = parse_input2("input1.txt");
    let solve2_result = solve2(solve2_input);
    println!("{:?}", solve2_result);
}
