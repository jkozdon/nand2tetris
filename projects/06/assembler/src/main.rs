use std::collections::HashMap;
use std::env;
use std::ffi::OsStr;
use std::fs;
use std::io::Write;
use std::path::Path;

fn cinst(line: &str) -> String {
    // Trim the comment
    let s = match line.find('/') {
        Some(v) => &line[0..v],
        _ => line,
    };
    let s = s.trim();

    // Find equal and semicolon (if there is one!)
    let eq = match s.find('=') {
        Some(v) => v + 1,
        None => 0,
    };
    let sc = match s.find(';') {
        Some(v) => v,
        None => s.len(),
    };

    let mut binary = "111".to_string();

    // Parse the comp bits
    match &s[eq..sc] {
        "0" => binary.push_str("0101010"),
        "1" => binary.push_str("0111111"),
        "-1" => binary.push_str("0111010"),
        "D" => binary.push_str("0001100"),
        "A" => binary.push_str("0110000"),
        "M" => binary.push_str("1110000"),
        "!D" => binary.push_str("0001101"),
        "!A" => binary.push_str("0110001"),
        "!M" => binary.push_str("1110001"),
        "-D" => binary.push_str("0001111"),
        "-A" => binary.push_str("0110011"),
        "-M" => binary.push_str("1110011"),
        "D+1" => binary.push_str("0011111"),
        "A+1" => binary.push_str("0110111"),
        "M+1" => binary.push_str("1110111"),
        "D-1" => binary.push_str("0001110"),
        "A-1" => binary.push_str("0110010"),
        "M-1" => binary.push_str("1110010"),
        "D+A" => binary.push_str("0000010"),
        "D+M" => binary.push_str("1000010"),
        "D-A" => binary.push_str("0010011"),
        "D-M" => binary.push_str("1010011"),
        "A-D" => binary.push_str("0000111"),
        "M-D" => binary.push_str("1000111"),
        "D&A" => binary.push_str("0000000"),
        "D&M" => binary.push_str("1000000"),
        "D|A" => binary.push_str("0010101"),
        "D|M" => binary.push_str("1010101"),
        _ => panic!("Bad cinstruction: {}", s),
    }

    // parse the destination bits:
    match s[0..eq].find('A') {
        Some(_) => binary.push_str("1"),
        None => binary.push_str("0"),
    };
    match s[0..eq].find('D') {
        Some(_) => binary.push_str("1"),
        None => binary.push_str("0"),
    };
    match s[0..eq].find('M') {
        Some(_) => binary.push_str("1"),
        None => binary.push_str("0"),
    };

    if sc + 1 <= s.len() {
        match &s[sc + 1..] {
            "JGT" => binary.push_str("001"),
            "JEQ" => binary.push_str("010"),
            "JGE" => binary.push_str("011"),
            "JLT" => binary.push_str("100"),
            "JNE" => binary.push_str("101"),
            "JLE" => binary.push_str("110"),
            "JMP" => binary.push_str("111"),
            _ => panic!("Bad cinstruction: {}", s),
        };
    } else {
        binary.push_str("000");
    }

    binary
}

fn ainst(line: &str, symbols: &mut HashMap<String, i32>, sym_count: &mut i32) -> String {
    let s = match line.find('/') {
        Some(v) => &line[1..v],
        _ => &line[1..],
    };
    let s = s.trim();
    let num: i32 = match s.parse() {
        Ok(num) => num,
        _ => match symbols.get(&s.to_string()) {
            Some(v) => *v,
            _ => {
                symbols.insert(s.to_string(), *sym_count);
                *sym_count += 1;
                *sym_count - 1
            }
        },
    };
    let mut binary = "0".to_string();
    for i in (0..15).rev() {
        let b = 1 << i;
        let digit = (num / b) % 2;
        binary = binary + &digit.to_string();
    }
    return binary;
}

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];

    let mut output: String = Path::new(file_path)
        .file_stem()
        .and_then(OsStr::to_str)
        .unwrap()
        .to_string();
    output.push_str(".hack");

    let mut output_file = fs::File::create(output)?;

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let mut symbols = HashMap::<String, i32>::new();
    symbols.insert("R0".to_string(), 0);
    symbols.insert("R1".to_string(), 1);
    symbols.insert("R2".to_string(), 2);
    symbols.insert("R3".to_string(), 3);
    symbols.insert("R4".to_string(), 4);
    symbols.insert("R5".to_string(), 5);
    symbols.insert("R6".to_string(), 6);
    symbols.insert("R7".to_string(), 7);
    symbols.insert("R8".to_string(), 8);
    symbols.insert("R9".to_string(), 9);
    symbols.insert("R10".to_string(), 10);
    symbols.insert("R11".to_string(), 11);
    symbols.insert("R12".to_string(), 12);
    symbols.insert("R13".to_string(), 13);
    symbols.insert("R14".to_string(), 14);
    symbols.insert("R15".to_string(), 15);
    symbols.insert("SCREEN".to_string(), 16384);
    symbols.insert("KBD".to_string(), 24576);
    symbols.insert("SP".to_string(), 0);
    symbols.insert("LCL".to_string(), 1);
    symbols.insert("ARG".to_string(), 2);
    symbols.insert("THIS".to_string(), 3);
    symbols.insert("THAT".to_string(), 4);

    let mut line_num = 0;
    for line in contents.lines() {
        let line = line.trim();
        // Skip emptpy lines and lines starting with comments
        if line.len() == 0 || line.chars().nth(0) == Some('/') {
            continue;
        }

        if line.chars().nth(0) == Some('(') {
            symbols.insert(line[1..line.len() - 1].to_string(), line_num);
            continue;
        }

        line_num += 1;
    }

    let mut sym_count = 16;
    for line in contents.lines() {
        let line = line.trim();
        // Skip emptpy lines and lines starting with comments
        if line.len() == 0 || line.chars().nth(0) == Some('/') || line.chars().nth(0) == Some('(') {
            continue;
        }
        let binary = if line.chars().nth(0) == Some('@') {
            ainst(line, &mut symbols, &mut sym_count)
        } else {
            cinst(line)
        };
        writeln!(output_file, "{binary}")?;
    }
    Ok(())
}
