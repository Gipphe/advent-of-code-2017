use regex::Regex;

mod input;

#[derive(Debug)]
#[derive(PartialEq)]
struct Program {
    name: String,
    weight: u32,
    listed_children: Vec<String>,
    children: Vec<Program>,
}
impl Program {
    fn sum_weight(&self) -> u32 {
        let children_weight = (&self.children)
            .into_iter()
            .map(|a| a.weight)
            .fold(0, |acc, c| acc + c);
        self.weight + children_weight
    }
}

fn get_children(line: &str) -> Option<Vec<String>> {
    lazy_static! {
        static ref CHILDREN: Regex = Regex::new(r"-> (?P<children>.+)$").unwrap();
    }
    let children_cap = CHILDREN.captures(&line);
    match children_cap {
        Some(children) => {
            let children = &children["children"];
            let reportedly = children.split(", ").collect::<Vec<&str>>();
            let res = reportedly.into_iter().map(|a| String::from(a)).collect::<Vec<String>>();
            Some(res)
        },
        None => None,
    }
}
fn get_names(input: &str) -> Vec<String> {
    lazy_static! {
        static ref NAME: Regex = Regex::new(r"^(\w+)").unwrap();
    }
    let mut res = vec![];
    for line in input.lines() {
        let name_cap = NAME.captures(line);
        match name_cap {
            Some(name) => {
                let name = String::from(&name[0]);
                res.push(name);
            },
            None => panic!("No name found"),
        }
    }
    res
}

fn get_root_name(input: &str) -> String {
    let mut children = vec![];
    for line in input.lines() {
        match get_children(line) {
            Some(childs) => {
                for child in childs {
                    children.push(child);
                }
            },
            None => {},
        };
    }
    let names = get_names(input);
    let root = names
        .into_iter()
        .find(|a| {
            match (&children).into_iter().find(|b| &a == b) {
                None => true,
                _ => false,
            }
        });
    match root {
        Some(name) => name,
        None => panic!("No root node found"),
    }
}
fn parse_line(line: &str) -> Program {
    lazy_static! {
        static ref NAME_AND_WEIGHT: Regex = Regex::new(r"^(?P<name>\w+) \((?P<weight>\d+)\)").unwrap();
        static ref CHILDREN: Regex = Regex::new(r"-> (?P<children>.+)$").unwrap();
    }

    let (name, weight) = match NAME_AND_WEIGHT.captures(line) {
        Some(caps) => (String::from(&caps["name"]), (&caps["weight"]).parse::<u32>()),
        None => panic!("No name or weight found for {}", line),
    };
    let weight = match weight {
        Ok(v) => v,
        Err(err) => panic!("{}: Weight not found for {}", err, line),
    };
    let children = match CHILDREN.captures(line) {
        Some(caps) => (&caps["children"])
            .split(", ")
            .map(|a| String::from(a))
            .collect(),
        None => vec![],
    };

    Program {
        name,
        weight,
        listed_children: children,
        children: vec![],
    }
}

fn connect_children(program: Program, children: &mut Vec<Program>) -> Program {
    let mut program = program;
    if program.listed_children.len() == 0 {
        return program;
    }
    for child_name in &program.listed_children {
        let i = children.into_iter().position(|a| &a.name == child_name).unwrap();
        let child = children.remove(i);
        program.children.push(connect_children(child, children));
    }
    program.listed_children.clear();
    program
}

fn build_tree(input: &str) -> Program {
    let root_name = get_root_name(input);
    let mut programs = input.lines().map(|a| parse_line(a)).collect::<Vec<Program>>();
    let i = (&programs).into_iter().position(|a| a.name == root_name).unwrap();
    let root = programs.remove(i);
    connect_children(root, &mut programs)
}

fn find_wrong_weight(root: Program) {
    for child in root.children {

    }
}

pub fn main() {
    let foo_input = "pbga (66)
xhth (57)
ebii (61)
havc (66)
ktlj (57)
fwft (72) -> ktlj, cntj, xhth
qoyq (66)
padx (45) -> pbga, havc, qoyq
tknk (41) -> ugml, padx, fwft
jptl (61)
ugml (68) -> gyxo, ebii, jptl
gyxo (61)
cntj (57)";
    let first = get_root_name(&input::INPUT);
    let second = build_tree(&foo_input);
    println!("Day 7-2: {:#?}", second);

    assert_eq!(first, "fbgguv", "Day 7-1 is incorrect: {}", first);
    println!("Day 7-1: {}", first);
}
