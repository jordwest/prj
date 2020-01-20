use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Debug)]
pub struct GitInfo {
    pub head_ref: String,
    pub changes: usize,
}

#[derive(Debug)]
pub struct Project {
    pub name: String,
    pub path: PathBuf,
}

#[derive(Debug)]
pub struct Remote {
    pub name: String,
    pub path: PathBuf,
}

pub struct Cache {
    pub remotes: HashMap<PathBuf, Remote>,
    pub projects: HashMap<PathBuf, Project>,
    pub git_info: HashMap<PathBuf, GitInfo>,
}

impl Cache {
    pub fn new() -> Cache {
        Cache {
            remotes: HashMap::new(),
            projects: HashMap::new(),
            git_info: HashMap::new(),
        }
    }
}
