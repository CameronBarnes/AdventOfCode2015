use std::{collections::HashMap, ops::{Not, Shl, Shr}};


enum Instruction {
    SET(String, String),
    AND(String, String, String),
    OR(String, String, String),
    XOR(String, String, String),
    NOT(String, String),
    LSHIFT(String, u16, String),
    RSHIFT(String, u16, String),
}

impl Instruction {
    fn is_ready(&self, wires: &HashMap<String, u16>) -> bool {
        match self {
            Instruction::SET(a, _) => present_or_number(a, wires),
            Instruction::AND(a, b, _) => {
                present_or_number(a, wires) && present_or_number(b, wires)
            },
            Instruction::OR(a, b, _) => {
                present_or_number(a, wires) && present_or_number(b, wires)
            },
            Instruction::XOR(a, b, _) => {
                present_or_number(a, wires) && present_or_number(b, wires)
            },
            Instruction::NOT(a, _) => present_or_number(a, wires),
            Instruction::LSHIFT(a, _, _) => present_or_number(a, wires),
            Instruction::RSHIFT(a, _, _) => present_or_number(a, wires),
        }
    }
}

fn parse_three(str: &str) -> (String, String, String) {
    let mut iter = str.split(' ');
    let a = iter.next().unwrap();
    iter.next().unwrap();
    let b = iter.next().unwrap();
    iter.next().unwrap();
    let result = iter.next().unwrap();

    (a.to_owned(), b.to_owned(), result.to_owned())
}

fn parse_set(str: &str) -> (String, String) {
    let mut iter = str.split(' ');
    let a = iter.next().unwrap();
    iter.next().unwrap();
    let b = iter.next().unwrap();

    (a.to_owned(), b.to_owned())
}

fn parse_not(str: &str) -> (String, String) {
    let mut iter = str.split(' ');
    iter.next().unwrap();
    let a = iter.next().unwrap();
    iter.next().unwrap();
    let b = iter.next().unwrap();

    (a.to_owned(), b.to_owned())
}

fn parse_line(str: &str) -> Instruction {

    if str.contains("AND") {
        let (a, b, result) = parse_three(str);
        Instruction::AND(a, b, result)
    } else if str.contains("OR") {
        let (a, b, result) = parse_three(str);
        Instruction::OR(a, b, result)
    } else if str.contains("XOR") {
        let (a, b, result) = parse_three(str);
        Instruction::XOR(a, b, result)
    } else if str.contains("LSHIFT") {
        let (a, b, result) = parse_three(str);
        Instruction::LSHIFT(a, b.parse().unwrap(), result)
    } else if str.contains("RSHIFT") {
        let (a, b, result) = parse_three(str);
        Instruction::RSHIFT(a, b.parse().unwrap(), result)
    } else if str.contains("NOT") {
        let (input, out) = parse_not(str);
        Instruction::NOT(input, out)
    } else { //Must be a SET instruction
        let (value, name) = parse_set(str);
        Instruction::SET(value, name)
    }

}

fn present_or_number(input: &str, wires: &HashMap<String, u16>) -> bool {
    input.parse::<u16>().is_ok() || wires.get(input).is_some()
}

fn get_or_number(input: &str, wires: &HashMap<String, u16>) -> u16 {

    if let Ok(num) = input.parse() {
        num
    } else {
        *wires.get(input).unwrap_or_else(|| panic!("Map value with the key {input} was not found"))
    }

}

fn do_instruction(instruction: &Instruction, wires: &mut HashMap<String, u16>) {

    match instruction {
            Instruction::SET(value, out) => {
                if wires.contains_key(out) {
                    return;
                }
                wires.insert(out.to_owned(), get_or_number(value, wires));
            },
            Instruction::AND(a, b, out) => {
                let a = get_or_number(a, wires);
                let b = get_or_number(b, wires);
                wires.insert(out.to_owned(), a & b);
            },
            Instruction::OR(a, b, out) => {
                let a = get_or_number(a, wires);
                let b = get_or_number(b, wires);
                wires.insert(out.to_owned(), a | b);
            },
            Instruction::XOR(a, b, out) => {
                let a = get_or_number(a, wires);
                let b = get_or_number(b, wires);
                wires.insert(out.to_owned(), a ^ b);
            },
            Instruction::NOT(input, out) => {
                let a = get_or_number(input, wires);
                wires.insert(out.to_owned(), a.not());
            },
            Instruction::LSHIFT(input, value, out) => {
                let a = get_or_number(input, wires);
                wires.insert(out.to_owned(), a.shl(value));
            },
            Instruction::RSHIFT(input, value, out) => {
                let a = get_or_number(input, wires);
                wires.insert(out.to_owned(), a.shr(value));
            },
        }

}

fn main() {
	
    let input = include_str!("./input.txt");

    let mut wires: HashMap<String, u16> = HashMap::new();
    let mut instructions: Vec<Instruction> = input.lines().map(parse_line).collect();

    while !instructions.is_empty() {
        instructions.retain(|instr| {
            if instr.is_ready(&wires) {
                do_instruction(instr, &mut wires);
                false
            } else {
                true
            }
        })
    }

    let a = wires.get("a").unwrap();

    println!("The value of the 'a' wire xis {a}");

    // That's part 1 done

    let mut wires: HashMap<String, u16> = HashMap::new();
    let mut instructions: Vec<Instruction> = input.lines().map(parse_line).collect();

    wires.insert("b".to_owned(), *a);

    while !instructions.is_empty() {
        instructions.retain(|instr| {
            if instr.is_ready(&wires) {
                do_instruction(instr, &mut wires);
                false
            } else {
                true
            }
        })
    }

    let a = wires.get("a").unwrap();

    println!("The value of the 'a' wire xis {a}");

}
