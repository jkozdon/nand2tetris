use std::env;
use std::ffi::OsStr;
use std::fs;
use std::path::Path;
use std::io::Write;

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
        writeln!(output_file, "// {line}")?;
        writeln!(output_file, "")?;

    }

    Ok(())
}
