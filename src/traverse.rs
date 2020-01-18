use crate::config::Config;
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub struct Project {
    name: String,
    path: PathBuf,
}

#[derive(Debug)]
pub struct Remote {
    name: String,
    path: PathBuf,
    projects: Vec<Project>,
}

#[derive(Debug)]
pub struct Root {
    path: PathBuf,
    remotes: Vec<Remote>,
}

#[derive(Debug)]
pub enum TraverseError {
    FailedToReadDir,
    InvalidRemoteName,
    InvalidFilename,
}

impl Root {
    pub fn traverse(config: &Config) -> Result<Self, TraverseError> {
        let path = config.root.as_path();
        let mut remotes: Vec<Remote> = Vec::new();

        let listing = path.read_dir().or(Err(TraverseError::FailedToReadDir))?;
        for entry_result in listing {
            let entry = entry_result.or(Err(TraverseError::FailedToReadDir))?;

            if entry.path().is_dir() {
                let remote = Remote::traverse(
                    entry
                        .file_name()
                        .into_string()
                        .or(Err(TraverseError::InvalidRemoteName))?,
                    &entry.path(),
                )?;

                remotes.push(remote);
            }
        }

        Ok(Root {
            path: config.root.clone(),
            remotes,
        })
    }
}

impl Remote {
    pub fn rec_traverse(
        &mut self,
        path: &Path,
        remaining_levels: usize,
    ) -> Result<(), TraverseError> {
        let listing = path.read_dir().or(Err(TraverseError::FailedToReadDir))?;

        let mut subdirs = vec![];

        for entry_result in listing {
            let entry = entry_result.or(Err(TraverseError::FailedToReadDir))?;

            let file_name = entry
                .file_name()
                .into_string()
                .or(Err(TraverseError::InvalidFilename))?;

            if file_name == ".git" {
                // We're in a project! Add it and skip any more traversal
                let project = Project {
                    name: path
                        .file_name()
                        .unwrap()
                        .to_str()
                        .ok_or(TraverseError::InvalidFilename)?
                        .to_string(),
                    path: path.to_path_buf(),
                };
                self.projects.push(project);
                return Ok(());
            }

            if entry.path().is_dir() {
                subdirs.push(entry.path());
            }
        }

        // We didn't find a project, so traverse all subdirectories
        for subdir in subdirs {
            if remaining_levels > 0 {
                self.rec_traverse(&subdir, remaining_levels - 1)?;
            }
        }

        Ok(())
    }

    pub fn traverse(name: String, path: &Path) -> Result<Self, TraverseError> {
        let mut remote = Remote {
            name,
            path: path.to_path_buf(),
            projects: vec![],
        };

        remote.rec_traverse(path, 2)?;

        Ok(remote)
    }
}
