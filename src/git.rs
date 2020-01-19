use git2::Repository;
use std::path::Path;

#[derive(Debug)]
pub struct GitProject {
    head_ref: String,
}

#[derive(Debug)]
pub enum GitError {
    FailedToOpen,
    FailedToReadHead,
}

impl GitProject {
    pub fn from_path(path: &Path) -> Result<GitProject, GitError> {
        use GitError::*;

        let repo = Repository::open(path).or(Err(FailedToOpen))?;
        let head = repo.head().or(Err(FailedToReadHead))?;

        let git_project = GitProject {
            head_ref: head
                .peel_to_commit()
                .or(Err(FailedToReadHead))?
                .summary()
                .ok_or(FailedToReadHead)?
                .to_string(),
        };
        Ok(git_project)
    }
}
