use std::fs;
use std::path::Path;
use std::collections::HashMap;
use itertools::Itertools;

#[derive(Debug)]
struct Monkey {
    id: u128,
    items: Vec<u128>,
    operation: Op,
    test: u128,
    tbranch: u128,
    fbranch: u128,
    monkey_business: u128
}

impl Monkey{
    fn new(monkey_id: u128, items_list: Vec<u128>, op: Op, test_n: u128, tbranch: u128, fbranch: u128) -> Self{
        Self {id: monkey_id,
              items: items_list,
              operation: op,
              test: test_n,
              tbranch,
              fbranch,
              monkey_business: 0}
    }
}

#[derive(Debug)]
struct Op {
    args: Vec<String>,
    opcode: String
}

impl Op {
    fn new(args: Vec<String>, opcode: String) -> Self{
        Self {args, opcode}
    }

    fn execute(&self, lookup: HashMap<&str, u128>) -> u128{
        let values = self.args.iter()
                              .map(|x| {
                                        let num = x.parse::<u128>();
                                        match num {
                                                    Ok(x) => x,
                                                    Err(_) => *lookup.get(&x[..])
                                                                     .expect("lookup failed")
                                                  }
                                        })
                              .collect::<Vec<_>>();
        match self.opcode.as_str() {
            "*" => values.iter().product(),
            "+" => values.iter().sum(),
            _ => panic!("opcode not found")
        }
    }
}

fn parse_op(op_string: &str) -> Op{
    let tokens = op_string.split(' ').collect::<Vec<_>>();
    let operand1 = tokens.get(5).expect("not found").to_string();
    let opcode = tokens.get(6).expect("not found").to_string();
    let operand2 = tokens.get(7).expect("not found").to_string();
    let args = vec![operand1, operand2];
    Op::new(args, opcode)
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

    let monkey_id = id_string.parse::<u128>()
                             .expect("can't parse id");

    let items_string = lines.next().expect("no line")
                                   .split(':')
                                   .collect::<Vec<_>>();
    let items_list = items_string.get(1)
                                 .expect("no list")
                                 .split(',')
                                 .map(|x| x.trim()
                                           .parse::<u128>()
                                           .expect("can't parse"))
                                 .collect::<Vec<_>>();

    let operation_string = lines.next()
                                .expect("no line");
    let op = parse_op(operation_string);

    let test_string = lines.next()
                           .expect("no line");
    let test_n = test_string.split(' ')
                            .collect::<Vec<_>>()
                            .get(5)
                            .expect("no number")
                            .parse::<u128>()
                            .expect("couldn't parse");

    let tbranch_string = lines.next()
                              .expect("no line");

    let tbranch = tbranch_string.split(' ')
                                .collect::<Vec<_>>()
                                .get(9)
                                .expect("no number")
                                .parse::<u128>()
                                .expect("couldn't parse");


    let fbranch_string = lines.next()
                              .expect("no line");

    let fbranch = fbranch_string.split(' ')
                                .collect::<Vec<_>>()
                                .get(9)
                                .expect("no number")
                                .parse::<u128>()
                                .expect("couldn't parse");

    Monkey::new(monkey_id, items_list, op, test_n, tbranch, fbranch)
}

fn turn(monkey: &mut Monkey, divide_by: u128, n: u128) -> HashMap<u128, Vec<u128>>{
    let items = &monkey.items;
    let mut moves: HashMap<u128, Vec<u128>> = HashMap::new();
    moves.insert(monkey.tbranch, Vec::new());
    moves.insert(monkey.fbranch, Vec::new());

    for item in items{
        let args = HashMap::from([("old", *item)]);
        let mut new_item = monkey.operation.execute(args);
        new_item /= divide_by;
        new_item %= n;
        if new_item % monkey.test == 0 {
            moves.get_mut(&monkey.tbranch).expect("no monkey").push(new_item);
        }else {
            moves.get_mut(&monkey.fbranch).expect("no monkey").push(new_item);
        }
        monkey.monkey_business += 1;
    }
    monkey.items = Vec::new();
    moves
}

fn round(monkeys: &mut Vec<Monkey>, divide_by: u128, n: u128){
    for i in 0..monkeys.len(){
        let monkey = monkeys.get_mut(i)
                            .expect("couldn't find monkey");
        let moves = turn(monkey, divide_by, n);
        for key in moves.keys(){
            let move_items = moves.get(key)
                                  .expect("couldn't find monkey");
            monkeys[(*key as usize)]
                   .items
                   .extend(move_items);
        }
    }
}

fn run_rounds(mut monkeys: Vec<Monkey>, num_rounds: i32, divide_by: u128) -> u128{
    let n = monkeys.iter().map(|m| m.test).product::<u128>();
    for _ in 0..num_rounds {
        round(&mut monkeys, divide_by, n);
    }
    monkeys.iter()
           .map(|m| m.monkey_business)
           .sorted()
           .rev()
           .take(2)
           .product()
}

fn solve1(monkeys: Vec<Monkey>) -> u128{
    run_rounds(monkeys, 20, 3)
}

fn solve2(mut monkeys: Vec<Monkey>) -> u128{
    run_rounds(monkeys, 10000, 1)
}

fn parse_input(fname: &str) -> Vec<Monkey>{
    let root_path = Path::new("/home/czw/Documents/2022/advent_of_code_2022/src/day11/");
    let fpath = root_path.join(fname);
    let string = fs::read_to_string(fpath).expect("file not found");
    string.split("\n\n")
          .map(parse_monkey)
          .collect::<Vec<_>>()
}

pub fn solve(){
    assert_eq!(10605, solve1(parse_input("test1.txt")));
    println!("{:?}", solve1(parse_input("input1.txt")));
    assert_eq!(2713310158, solve2(parse_input("test1.txt")));
    println!("{:?}", solve2(parse_input("input1.txt")));
}
