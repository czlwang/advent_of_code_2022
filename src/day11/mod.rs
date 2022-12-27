use std::fs;
use std::path::Path;

#[derive(Debug)]
struct Monkey {
    id: i32,
    items: Vec<i32>,
    operation: String,
    test: String,
    tbranch: i32,
    fbranch: i32
}

impl Monkey{
    fn new(monkey_id: i32, items_list: Vec<i32>, tbranch: i32, fbranch: i32) -> Self{
        Self {id: monkey_id,
              items: items_list,
              operation: "a".to_string(),
              test: "b".to_string(),
              tbranch,
              fbranch}
    }
}

fn parse_monkey(monkey_string: &str) -> Monkey{
    let mut lines = monkey_string.split('\n');
    let id_line = lines.next().expect("no line")
                              .split(' ')
                              .collect::<Vec<_>>();
    let mut id_string = id_line.get(1)
                               .expect("no id")
                               .to_string();
    id_string.pop();

    let monkey_id = id_string.parse::<i32>()
                             .expect("can't parse id");

    let items_string = lines.next().expect("no line")
                                   .split(':')
                                   .collect::<Vec<_>>();
    let items_list = items_string.get(1)
                                 .expect("no list")
                                 .split(',')
                                 .map(|x| x.trim()
                                           .parse::<i32>()
                                           .expect("can't parse"))
                                 .collect::<Vec<_>>();

    let operation_string = lines.next()
                                .expect("no line");

    let test_string = lines.next()
                           .expect("no line");

    let tbranch_string = lines.next()
                              .expect("no line");

    let tbranch = tbranch_string.split(' ')
                                .collect::<Vec<_>>()
                                .get(9)
                                .expect("no number")
                                .parse::<i32>()
                                .expect("couldn't parse");


    let fbranch_string = lines.next()
                              .expect("no line");

    let fbranch = fbranch_string.split(' ')
                                .collect::<Vec<_>>()
                                .get(9)
                                .expect("no number")
                                .parse::<i32>()
                                .expect("couldn't parse");

    Monkey::new(monkey_id, items_list, tbranch, fbranch)
}

fn parse_input(fname: &str){
    let root_path = Path::new("/home/czw/Documents/2022/advent_of_code_2022/src/day11/");
    let fpath = root_path.join(fname);
    let string = fs::read_to_string(fpath).expect("file not found");
    let monkeys = string.split("\n\n")
                        .map(parse_monkey)
                        .collect::<Vec<_>>();
    println!("{:?}", monkeys);
}

pub fn solve(){
    parse_input("test1.txt");
    println!("hello");
}
