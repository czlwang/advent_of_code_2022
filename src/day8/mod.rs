use std::fs;
use std::path::Path;
use std::collections::HashSet;

fn parse_input(fname: &str) -> Vec<Vec<i32>>{
    let root_path = Path::new("/home/czw/Documents/2022/advent_of_code_2022/src/day8");
    let fpath = root_path.join(fname);
    fs::read_to_string(fpath)
                     .expect("no such file")
                     .trim()
                     .split('\n')
                     .map(|line| 
                          {
                              let chars = line.chars();
                              chars.map(|c| c.to_string()
                                             .parse::<i32>()
                                             .expect("parse to int failure"))
                                   .collect()
                          })
                      .collect()
}

fn solve1(grid: Vec<Vec<i32>>) -> i32{
    let mut left: Vec<Vec<i32>> = Vec::new();
    let mut right: Vec<Vec<i32>> = Vec::new();
    let mut up: Vec<Vec<i32>> = Vec::new();
    let mut down: Vec<Vec<i32>> = Vec::new();

    let nrows = grid.len();
    let ncols = grid[0].len();

    for row in 0..nrows{
        let mut left_max = -1;
        let mut right_max = -1;

        left.push(Vec::new());
        right.push(Vec::new());
        for col in 0..ncols{
            left[row].push(left_max);
            right[row].insert(0, right_max);
            left_max = std::cmp::max(left_max, grid[row][col]);
            right_max = std::cmp::max(right_max, grid[row][(ncols-1) - col]);
        }
    }

    for col in 0..ncols{
        let mut up_max = -1;
        let mut down_max = -1;

        if col==0 {
            for _ in 0..nrows{
                up.push(Vec::new());
                down.push(Vec::new());
            }
        }

        for row in 0..nrows{
            up[row].push(up_max);
            down[(nrows-1) - row].push(down_max);
            up_max = std::cmp::max(up_max, grid[row][col]);
            down_max = std::cmp::max(down_max, grid[(nrows-1) - row][col]);
            //right_max = std::cmp::max(right_max, grid[(nrows-1) - row][(ncols-1) - col]);
        }
    }

    let mut visible = 0;
    for row in 0..nrows{
        for col in 0..ncols{
            if grid[row][col] > left[row][col]  ||
               grid[row][col] > right[row][col] ||
               grid[row][col] > up[row][col] ||
               grid[row][col] > down[row][col]
               {
                visible += 1;
               }
        }
    }
    visible
}

fn get_score(grid: &Vec<Vec<i32>>, row: usize, col: usize) -> i32{
    let nrows = grid.len();
    let ncols = grid[0].len();

    let mut left_score = 0;
    for i in 0..col{
        left_score += 1;
        if grid[row][col-(i+1)] >= grid[row][col]{
            break;
        }
    }
    let mut right_score = 0;
    for i in (col+1)..ncols{
        right_score += 1;
        if grid[row][i] >= grid[row][col]{
            break;
        }
    }
    let mut up_score = 0;
    for i in 0..row{
        up_score += 1;
        if grid[row-(i+1)][col] >= grid[row][col]{
            break;
        }
    }

    let mut down_score = 0;
    for i in (row+1)..nrows{
        down_score += 1;
        if grid[i][col] >= grid[row][col]{
            break;
        }
    }

    up_score*down_score*left_score*right_score
}

fn solve2(grid: Vec<Vec<i32>>) -> i32{
    let nrows = grid.len();
    let ncols = grid[0].len();

    let mut score = 0;
    for row in 0..nrows{
        for col in 0..ncols{
            score = std::cmp::max(get_score(&grid, row, col), score);
        }
    }
    score
}

pub fn solve(){
    assert_eq!(21,solve1(parse_input("test1.txt")));
    println!("{:?}", solve1(parse_input("input1.txt")));
    assert_eq!(8,solve2(parse_input("test1.txt")));
    println!("{:?}", solve2(parse_input("input1.txt")));
}
