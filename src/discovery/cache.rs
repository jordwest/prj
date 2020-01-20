use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone)]
pub struct VcsInfo {
    pub last_commit_summary: String,
    pub current_branch_name: String,
    pub uncommitted_changes: usize,
}

#[derive(Debug, Clone)]
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
    update_count: i32,
}

impl Cache {
    pub fn new() -> Cache {
        Cache {
            remotes: HashMap::new(),
            projects: HashMap::new(),
            vcs_info: HashMap::new(),
            update_count: 0,
        }
    }

    /// Share this cache among threads.
    /// Takes ownership of the cache and provides a clonable client to communicate with it concurrently
    pub fn share(self) -> CacheClient {
        let arc = Arc::new(Mutex::new(self));

        CacheClient {
            cache: arc,
            last_checked_update: 0,
        }
    }
}

#[derive(Clone)]
pub struct CacheClient {
    cache: Arc<Mutex<Cache>>,
    last_checked_update: i32,
}

impl CacheClient {
    pub fn get_projects(&self) -> Vec<Project> {
        let cache = self.cache.lock().unwrap();

        (*cache).projects.values().cloned().collect()
    }

    pub fn get_vcs_info(&self, path: &Path) -> std::option::Option<VcsInfo> {
        let cache = self.cache.lock().unwrap();

        (*cache).vcs_info.get(path).map(|v| v.clone())
    }

    pub fn add_vcs_info(&mut self, path: &Path, val: VcsInfo) {
        let mut cache = self.cache.lock().unwrap();
        cache.update_count += 1;
        (*cache).vcs_info.insert(path.to_path_buf(), val);
    }

    pub fn has_new_data(&mut self) -> bool {
        let cache = self.cache.lock().unwrap();

        if self.last_checked_update < cache.update_count {
            self.last_checked_update = cache.update_count;
            return true;
        }
        false
    }
}
