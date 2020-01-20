use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Debug)]
pub struct VcsInfo {
    pub last_commit_summary: String,
    pub current_branch_name: String,
    pub uncommitted_changes: usize,
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
    pub vcs_info: HashMap<PathBuf, VcsInfo>,
}

impl Cache {
    pub fn new() -> Cache {
        Cache {
            remotes: HashMap::new(),
            projects: HashMap::new(),
            vcs_info: HashMap::new(),
        }
    }
}
