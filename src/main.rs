use std::env;
mod day1;
mod day2;
mod day3;

fn main() {
    let args: Vec<String> = env::args().collect();
    let day: i32 = args[1].parse().unwrap();
    match day{
        1 => day1::solve(),
        2 => day2::solve(),
        3 => day3::solve(),
        _ => println!("Invalid day"),
    }
}