use std::env;
use std::ffi::OsStr;
use std::fs;
use std::io;
use std::io::Write;
use std::path::Path;

fn op_push(file: &str, line: &str, output: &mut fs::File) -> Result<(), io::Error> {
    writeln!(output, "// {line}")?;

    let num = line.split(" ").nth(2).expect("num");
    let num: i32 = num.trim().parse().expect("Wanted a number");
    match line.split(" ").nth(1) {
        Some("constant") => {
            writeln!(output, "@{num}")?;
            writeln!(output, "    D=A")?;
        }
        Some("static") => {
            writeln!(output, "@{file}.{num}")?;
            writeln!(output, "    D=M")?;
        }
        Some(v) => println!("push: {v}"),
        _ => println!("{line}"),
    };

    writeln!(output, "    @SP")?;
    writeln!(output, "    A=M")?;
    writeln!(output, "    M=D")?;
    writeln!(output, "    @SP")?;
    writeln!(output, "    M=M+1")?;
    return Ok(());
}

fn op_pop(file: &str, line: &str, output: &mut fs::File) -> Result<(), io::Error> {
    writeln!(output, "// {line}")?;
    writeln!(output, "    @SP")?;
    writeln!(output, "    M=M-1")?;
    writeln!(output, "    A=M")?;
    writeln!(output, "    D=M")?;

    let num = line.split(" ").nth(2).expect("num");
    let num: i32 = num.trim().parse().expect("Wanted a number");
    match line.split(" ").nth(1) {
        Some("static") => {
            writeln!(output, "@{file}.{num}")?;
        }
        Some(v) => println!("pop: {v}"),
        _ => println!("{line}"),
    };
    writeln!(output, "    M=D")?;

    return Ok(());
}

fn bool_op(jump: &str, output: &mut fs::File, count: u32) -> Result<(), io::Error> {
    writeln!(output, "    @JMP.{count}")?;
    writeln!(output, "    D;{jump}")?;
    writeln!(output, "    @SP")?;
    writeln!(output, "    A=M")?;
    writeln!(output, "    M=0")?;
    writeln!(output, "    @DONE.{count}")?;
    writeln!(output, "    0;JMP")?;
    writeln!(output, "(JMP.{count})")?;
    writeln!(output, "    @SP")?;
    writeln!(output, "    A=M")?;
    writeln!(output, "    M=-1")?;
    writeln!(output, "(DONE.{count})")?;
    return Ok(());
}

fn uni_op(line: &str, output: &mut fs::File) -> Result<(), io::Error> {
    writeln!(output, "// {line}")?;
    writeln!(output, "    @SP")?;
    writeln!(output, "    M=M-1")?;
    writeln!(output, "    A=M")?;
    match line.split(" ").nth(0) {
        Some("neg") => writeln!(output, "    M=-M")?,
        Some("not") => writeln!(output, "    M=!M")?,
        _ => println!("uni_op: {line}"),
    }
    writeln!(output, "    @SP")?;
    writeln!(output, "    M=M+1")?;
    return Ok(());
}

fn bin_op(line: &str, output: &mut fs::File, count: u32) -> Result<(), io::Error> {
    writeln!(output, "// {line}")?;
    writeln!(output, "    @SP")?;
    writeln!(output, "    M=M-1")?;
    writeln!(output, "    A=M")?;
    writeln!(output, "    D=M")?;
    writeln!(output, "    @SP")?;
    writeln!(output, "    M=M-1")?;
    writeln!(output, "    @SP")?;
    writeln!(output, "    A=M")?;
    match line.split(" ").nth(0) {
        Some("add") => writeln!(output, "    M=M+D")?,
        Some("sub") => writeln!(output, "    M=M-D")?,
        Some("and") => writeln!(output, "    M=M&D")?,
        Some("or") => writeln!(output, "    M=M|D")?,
        Some("eq") => {
            writeln!(output, "    D=M-D")?;
            bool_op("JEQ", output, count)?;
        }
        Some("lt") => {
            writeln!(output, "    D=M-D")?;
            bool_op("JLT", output, count)?;
        }
        Some("gt") => {
            writeln!(output, "    D=M-D")?;
            bool_op("JGT", output, count)?;
        }
        _ => println!("bin_op: {line}"),
    }
    writeln!(output, "    @SP")?;
    writeln!(output, "    M=M+1")?;
    return Ok(());
}

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];

    let mut output: String = Path::new(file_path)
        .file_stem()
        .and_then(OsStr::to_str)
        .unwrap()
        .to_string();
    let file = output.clone();
    output.push_str(".asm");

    let mut output = fs::File::create(output)?;

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let mut count = 0;
    for line in contents.lines() {
        let line = line.trim();
        // Skip emptpy lines and lines starting with comments
        if line.len() == 0 || line.chars().nth(0) == Some('/') {
            continue;
        }

        count = count + 1;

        match line.split(" ").nth(0) {
            Some("push") => op_push(&file, line, &mut output)?,
            Some("pop") => op_pop(&file, line, &mut output)?,
            Some("add") => bin_op(line, &mut output, count)?,
            Some("sub") => bin_op(line, &mut output, count)?,
            Some("and") => bin_op(line, &mut output, count)?,
            Some("or") => bin_op(line, &mut output, count)?,
            Some("eq") => bin_op(line, &mut output, count)?,
            Some("lt") => bin_op(line, &mut output, count)?,
            Some("gt") => bin_op(line, &mut output, count)?,
            Some("neg") => uni_op(line, &mut output)?,
            Some("not") => uni_op(line, &mut output)?,
            _ => println!("op: {line}"),
        };
        writeln!(output, "")?;
    }

    Ok(())
}
