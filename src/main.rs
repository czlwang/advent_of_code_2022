use std::env;
mod day1;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}",args);
    let day: i32 = args[1].parse().unwrap();
    match day{
        1 => day1::solve(),
        _ => println!("Invalid day"),
    }
}