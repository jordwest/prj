use crate::config::{Config, WriteError};
use dirs::home_dir;
use std::io;
use std::path::PathBuf;

#[derive(Debug)]
pub enum ConfigureError {
    HomeDirNotFound,
    PathNotParsable,
    EndOfInput,
    UserCancelled,
    WriteError(WriteError),
}

fn ask(prompt: &str, default: &str) -> Result<String, ConfigureError> {
    let stdin = io::stdin();

    let mut raw_result = String::new();

    println!("{} [{}]", prompt, default);
    stdin
        .read_line(&mut raw_result)
        .or(Err(ConfigureError::EndOfInput))?;

    let result = raw_result.trim();

    if result.len() == 0 {
        return Ok(default.to_string());
    }
    Ok(result.to_string())
}

pub fn configure() -> Result<(), ConfigureError> {
    let home_dir = home_dir().ok_or(ConfigureError::HomeDirNotFound)?;

    let existing_config = Config::autoload().ok();
    if let Some(_) = existing_config {
        let overwrite = ask(
            "I found an existing config file, do you want to overwrite it?",
            "N",
        )?;
        if overwrite != "y" && overwrite != "Y" {
            return Err(ConfigureError::UserCancelled);
        }
    }

    println!(include_str!("./configure_root_doc.txt"));

    let default_root = match existing_config {
        Some(ref c) => c.root.clone(),
        None => home_dir.join("src"),
    };

    let root = ask(
        "Where will your projects live?",
        default_root
            .to_str()
            .ok_or(ConfigureError::PathNotParsable)?,
    )?;

    let location = match existing_config {
        Some(ref c) => c.location.clone(),
        None => home_dir.join(".prj"),
    };

    let root = PathBuf::from(root);
    let config = Config { root, location };

    config
        .write_config()
        .or_else(|e| Err(ConfigureError::WriteError(e)))
}
