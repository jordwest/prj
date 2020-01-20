use super::cache::{Cache, Project};
use crate::config::Config;
use std::path::Path;

#[derive(Debug)]
pub enum TraverseError {
    FailedToReadDir,
    InvalidFilename,
}

impl Cache {
    pub fn find_all_projects(&mut self, config: &Config) -> Result<(), TraverseError> {
        let path = config.root.as_path();

        self.rec_traverse_dir(path, 3)?;

        Ok(())
    }

    fn rec_traverse_dir(
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
                self.projects.insert(path.to_path_buf(), project);
                return Ok(());
            }

            if entry.path().is_dir() {
                subdirs.push(entry.path());
            }
        }

        // We didn't find a project, so traverse all subdirectories
        for subdir in subdirs {
            if remaining_levels > 0 {
                self.rec_traverse_dir(&subdir, remaining_levels - 1)?;
            }
        }

        Ok(())
    }
}
