use std::env;
use std::ffi::OsStr;
use std::fs;
use std::io;
use std::io::Write;
use std::path::Path;

fn push(line: &str, output_file: &mut fs::File) -> Result<(), io::Error> {
    writeln!(output_file, "// {line}")?;
    match line.split(" ").nth(1) {
        Some("constant") => (),
        Some(v) => println!("{v}"),
        _ => println!("{line}"),
    };

    match line.split(" ").nth(2) {
        Some(v) => writeln!(output_file, "@{v}")?,
        _ => println!("{line}"),
    };

    writeln!(output_file, "D=A")?;
    writeln!(output_file, "@SP")?;
    writeln!(output_file, "A=M")?;
    writeln!(output_file, "M=D")?;
    writeln!(output_file, "@SP")?;
    writeln!(output_file, "M=M+1")?;
    return Ok(());
}

fn add(line: &str, output_file: &mut fs::File) -> Result<(), io::Error> {
    writeln!(output_file, "// {line}")?;
    writeln!(output_file, "@SP")?;
    writeln!(output_file, "M=M-1")?;
    writeln!(output_file, "A=M")?;
    writeln!(output_file, "D=M")?;
    writeln!(output_file, "@SP")?;
    writeln!(output_file, "M=M-1")?;
    writeln!(output_file, "@SP")?;
    writeln!(output_file, "A=M")?;
    writeln!(output_file, "M=M+D")?;
    writeln!(output_file, "@SP")?;
    writeln!(output_file, "M=M+1")?;
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

    for line in contents.lines() {
        let line = line.trim();
        // Skip emptpy lines and lines starting with comments
        if line.len() == 0 || line.chars().nth(0) == Some('/') {
            continue;
        }

        match line.split(" ").nth(0) {
            Some("push") => push(line, &mut output_file)?,
            Some("add") => add(line, &mut output_file)?,
            _ => println!("{line}"),
        };
        writeln!(output_file, "")?;
    }

    Ok(())
}
