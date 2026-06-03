use anyhow::{Result, bail};
use std::fs;
use std::io::{self, Read, Write};
use std::path::PathBuf;

pub fn read_text_input(
    inline: Option<String>,
    file: Option<PathBuf>,
    stdin: bool,
    choices: &'static str,
) -> Result<String> {
    match (inline, file, stdin) {
        (Some(value), None, false) => Ok(value),
        (None, Some(path), false) => Ok(fs::read_to_string(path)?),
        (None, None, true) => read_stdin(),
        _ => bail!("exactly one of {choices} is required"),
    }
}

pub fn read_stdin() -> Result<String> {
    let mut value = String::new();
    io::stdin().read_to_string(&mut value)?;
    Ok(value)
}

pub fn prompt_line(prompt: &str) -> Result<String> {
    print!("{prompt}");
    io::stdout().flush()?;
    let mut line = String::new();
    io::stdin().read_line(&mut line)?;
    Ok(line.trim().to_string())
}
