use std::env;
use std::ffi::OsStr;
use std::fs;
use std::io;
use std::io::Write;
use std::path::Path;

fn op_push(line: &str, output_file: &mut fs::File) -> Result<(), io::Error> {
    writeln!(output_file, "// {line}")?;
    match line.split(" ").nth(1) {
        Some("constant") => (),
        Some(v) => println!("push: {v}"),
        _ => println!("{line}"),
    };

    match line.split(" ").nth(2) {
        Some(v) => writeln!(output_file, "@{v}")?,
        _ => println!("{line}"),
    };

    writeln!(output_file, "    D=A")?;
    writeln!(output_file, "    @SP")?;
    writeln!(output_file, "    A=M")?;
    writeln!(output_file, "    M=D")?;
    writeln!(output_file, "    @SP")?;
    writeln!(output_file, "    M=M+1")?;
    return Ok(());
}

fn bool_op(jump: &str, output_file: &mut fs::File, count: u32) -> Result<(), io::Error> {
    writeln!(output_file, "    @JMP.{count}")?;
    writeln!(output_file, "    D;{jump}")?;
    writeln!(output_file, "    @SP")?;
    writeln!(output_file, "    A=M")?;
    writeln!(output_file, "    M=0")?;
    writeln!(output_file, "    @DONE.{count}")?;
    writeln!(output_file, "    0;JMP")?;
    writeln!(output_file, "(JMP.{count})")?;
    writeln!(output_file, "    @SP")?;
    writeln!(output_file, "    A=M")?;
    writeln!(output_file, "    M=-1")?;
    writeln!(output_file, "(DONE.{count})")?;
    return Ok(());
}

fn uni_op(line: &str, output_file: &mut fs::File) -> Result<(), io::Error> {
    writeln!(output_file, "// {line}")?;
    writeln!(output_file, "    @SP")?;
    writeln!(output_file, "    M=M-1")?;
    writeln!(output_file, "    A=M")?;
    match line.split(" ").nth(0) {
        Some("neg") => writeln!(output_file, "    M=-M")?,
        Some("not") => writeln!(output_file, "    M=!M")?,
        _ => println!("uni_op: {line}"),
    }
    writeln!(output_file, "    @SP")?;
    writeln!(output_file, "    M=M+1")?;
    return Ok(());
}

fn bin_op(line: &str, output_file: &mut fs::File, count: u32) -> Result<(), io::Error> {
    writeln!(output_file, "// {line}")?;
    writeln!(output_file, "    @SP")?;
    writeln!(output_file, "    M=M-1")?;
    writeln!(output_file, "    A=M")?;
    writeln!(output_file, "    D=M")?;
    writeln!(output_file, "    @SP")?;
    writeln!(output_file, "    M=M-1")?;
    writeln!(output_file, "    @SP")?;
    writeln!(output_file, "    A=M")?;
    match line.split(" ").nth(0) {
        Some("add") => writeln!(output_file, "    M=M+D")?,
        Some("sub") => writeln!(output_file, "    M=M-D")?,
        Some("and") => writeln!(output_file, "    M=M&D")?,
        Some("or") => writeln!(output_file, "    M=M|D")?,
        Some("eq") => {
            writeln!(output_file, "    D=M-D")?;
            bool_op("JEQ", output_file, count)?;
        }
        Some("lt") => {
            writeln!(output_file, "    D=M-D")?;
            bool_op("JLT", output_file, count)?;
        }
        Some("gt") => {
            writeln!(output_file, "    D=M-D")?;
            bool_op("JGT", output_file, count)?;
        }
        _ => println!("bin_op: {line}"),
    }
    writeln!(output_file, "    @SP")?;
    writeln!(output_file, "    M=M+1")?;
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
    output.push_str(".asm");

    let mut output_file = fs::File::create(output)?;

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
            Some("push") => op_push(line, &mut output_file)?,
            Some("add") => bin_op(line, &mut output_file, count)?,
            Some("sub") => bin_op(line, &mut output_file, count)?,
            Some("and") => bin_op(line, &mut output_file, count)?,
            Some("or") => bin_op(line, &mut output_file, count)?,
            Some("eq") => bin_op(line, &mut output_file, count)?,
            Some("lt") => bin_op(line, &mut output_file, count)?,
            Some("gt") => bin_op(line, &mut output_file, count)?,
            Some("neg") => uni_op(line, &mut output_file)?,
            Some("not") => uni_op(line, &mut output_file)?,
            _ => println!("op: {line}"),
        };
        writeln!(output_file, "")?;
    }

    Ok(())
}
