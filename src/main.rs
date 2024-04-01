use std::fs;
use std::env;
fn main() {
    let argv: Vec<String> = env::args().collect();
    let file = argv[1].to_string();
    let content = fs::read_to_string(file).expect("Cannot find file!");
    for line in content.lines() {
        if line.is_empty() {
            continue;
        }
        let (op, rest) = line.split_once(" ").unwrap();
        let (rsrc, rdest) = rest.split_once(", ").unwrap();
        let (rsrc, rdest) = if op == "load" || op == "stor" {(rdest, rsrc)} else {(rsrc, rdest)};

        let rdest = if rdest.starts_with("r") {
            format!("{:04b}", rdest[1..].trim().parse::<u32>().unwrap())
        } else {
            if rdest.trim().chars().all(|c| c.is_numeric()) {
                format!("{:04b}", rdest.trim().parse::<u32>().unwrap())
            } else {
                match rdest.trim() {
                    "eq" => "0000",
                    "ne" => "0001",
                    "ge" => "1101",
                    "cs" => "0010",
                    "cc" => "0011",
                    "hi" => "0100",
                    "ls" => "0101",
                    "lo" => "1010",
                    "hs" => "1011",
                    "gt" => "0110",
                    "le" => "0111",
                    "fs" => "1000",
                    "fc" => "1001",
                    "lt" => "1100",
                    "uc" => "1110",
                    _ => "1111",
                }.to_string()
            }
        };

        let rsrc = if rsrc.starts_with("r") {
            format!("{:04b}", rsrc[1..].trim().parse::<u32>().unwrap())
        } else if op == "lshi" {
            format!("{:04b}", rsrc.trim().parse::<u32>().unwrap())
        } else {
            format!("{:08b}", rsrc.trim().parse::<u32>().unwrap())
        };

        let op = match op {
            "add" => ("0000", "0101"),
            "addi" => ("0101", ""),
            "sub" => ("0000", "1001"),
            "subi" => ("1001", ""),
            "cmp" => ("0000", "1011"),
            "cmpi" => ("1011", ""),
            "and" => ("0000", "0001"),
            "andi" => ("0001", ""),
            "or" => ("0000", "0010"),
            "ori" => ("0010", ""),
            "xor" => ("0000", "0011"),
            "xori" => ("0011", ""),
            "mov" => ("0000", "1101"),
            "movi" => ("1101", ""),
            "lsh" => ("1000", "0100"),
            "lshi" => ("1000", "0000"),
            "lui" => ("1111", ""),
            "load" => ("0100", "0000"),
            "stor" => ("0100", "0100"),
            "bcond" => ("1100", ""),
            "jcond" => ("0100", "1100"),
            "jal" => ("0100", "1000"),
            _ => panic!("Invalid command: {op}"),
        };
        println!("16'b{}{}{}{}  // {}", op.0, rdest, op.1, rsrc, line);
    }
}
