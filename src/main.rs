use std::fs;
fn main() {
    let content = fs::read_to_string("input.asm").unwrap();
    for line in content.lines() {
        let (op, rest) = line.split_once(" ").unwrap();
        let (r2, r1) = rest.split_once(", ").unwrap();
        let r1 = if r1.starts_with("r") { &r1[1..] } else { r1 };
        let r2 = if r2.starts_with("r") {
            &r2[1..]
        } else {
            if r2.trim().chars().all(|c| c.is_numeric()) {
                r2
            } else {
                match r2.trim() {
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
                }
            }
        };

        let r1: u32 = r1.trim().parse().unwrap();
        let r2: u32 = r2.trim().parse().unwrap();

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
            _ => panic!("Invalid Command: {op}"),
        };

        let mut holding = 4;

        let r1 = format!("{:0holding$b}", r1);

        if op.1.is_empty() {
            holding = 8;
        }

        let r2 = format!("{:0holding$b}", r2);

        println!("{}{}{}{}", op.0, r1, op.1, r2);
    }
}
