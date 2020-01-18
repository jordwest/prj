use dirs::home_dir;
use serde_derive::{Deserialize, Serialize};
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::path::PathBuf;

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    /// Location of the configuration file
    #[serde(skip)]
    pub location: PathBuf,

    pub root: PathBuf,
}

use toml;

#[derive(Debug)]
pub enum ReadError {
    Missing,
    IoError,
    HomeDirNotFound,
    ParseError,
}

#[derive(Debug)]
pub enum WriteError {
    IoError(std::io::Error),
    SerializeError,
}

impl Config {
    /// Try to find the config file and load it
    pub fn autoload() -> Result<Config, ReadError> {
        let home = home_dir().ok_or(ReadError::HomeDirNotFound)?;
        let expected_locations = vec![
            home.join(PathBuf::from(".prj.toml")),
            home.join(PathBuf::from(".prj")),
        ];
        for path in expected_locations {
            if path.exists() {
                println!("{:?}", path);
                return Config::load_from(&path);
            }
        }
        Err(ReadError::Missing)
    }

    pub fn load_from(path_to_config: &Path) -> Result<Config, ReadError> {
        if !path_to_config.exists() {
            return Err(ReadError::Missing);
        }

        let mut f = File::open(path_to_config).or(Err(ReadError::IoError))?;
        // read the whole file
        let mut buffer = Vec::new();
        f.read_to_end(&mut buffer).or(Err(ReadError::IoError))?;

        // Parse it
        let mut config: Config = toml::from_slice(&buffer).or(Err(ReadError::ParseError))?;
        config.location = path_to_config.into();

        Ok(config)
    }

    pub fn write_config(&self) -> Result<(), WriteError> {
        println!("Writing to {:?}", self.location);

        let output = toml::to_vec(self).or(Err(WriteError::SerializeError))?;
        let mut f = File::create(&self.location).or_else(|e| Err(WriteError::IoError(e)))?;

        f.write_all(&output)
            .or_else(|e| Err(WriteError::IoError(e)))
    }
}
