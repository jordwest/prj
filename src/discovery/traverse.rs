use crate::config::Config;
use std::path::{Path, PathBuf};

pub struct Traverser {
    queue: Vec<(PathBuf, u8)>,
    max_nesting: u8,
}

#[derive(Debug)]
pub enum TraverseError {
    FailedToReadDir,
    InvalidFilename,
    Finished,
}

impl Iterator for Traverser {
    type Item = PathBuf;

    fn next(&mut self) -> Option<PathBuf> {
        loop {
            match self.find_project() {
                // We found a project
                Ok(path) => return Some(path),

                // Nothing left to search
                Err(TraverseError::Finished) => return None,

                // Ignore other errors, keep searching
                Err(_) => (),
            };
        }
    }
}

impl Traverser {
    pub fn new(path: &Path, max_nesting: u8) -> Self {
        let mut queue = Vec::with_capacity(60);
        queue.push((path.to_path_buf(), 0));

        Traverser { queue, max_nesting }
    }

    fn find_project(&mut self) -> Result<PathBuf, TraverseError> {
        while let Some((path, nesting_level)) = self.queue.pop() {
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
                    return Ok(path.to_path_buf());
                }

                if entry.path().is_dir() {
                    subdirs.push(entry.path());
                }
            }

            // We mustn't have found a project in this directory, queue up the subdirectories
            for subdir in subdirs {
                if nesting_level < self.max_nesting {
                    self.queue.push((subdir.to_path_buf(), nesting_level + 1));
                }
            }
        }

        Err(TraverseError::Finished)
    }
}

pub fn find_all_projects(
    config: &Config,
    callback: fn(PathBuf) -> (),
) -> Result<(), TraverseError> {
    let path = config.root.as_path();

    rec_traverse_dir(path, 3, callback)?;

    Ok(())
}

fn rec_traverse_dir(
    path: &Path,
    remaining_levels: u8,
    callback: fn(PathBuf) -> (),
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
            callback(path.to_path_buf());
            // let project = Project {
            //     name: path
            //         .file_name()
            //         .unwrap()
            //         .to_str()
            //         .ok_or(TraverseError::InvalidFilename)?
            //         .to_string(),
            //     path: path.to_path_buf(),
            // };
            // self.projects.insert(path.to_path_buf(), project);
            return Ok(());
        }

        if entry.path().is_dir() {
            subdirs.push(entry.path());
        }
    }

    // We didn't find a project, so traverse all subdirectories
    for subdir in subdirs {
        if remaining_levels > 0 {
            rec_traverse_dir(&subdir, remaining_levels - 1, callback)?;
        }
    }

    Ok(())
}
