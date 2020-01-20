use super::cache::GitInfo;
use git2::Repository;
use std::path::Path;

#[derive(Debug)]
pub enum GitError {
    FailedToOpen,
    FailedToReadHead,
    FailedToReadStatus,
}

impl super::cache::GitInfo {
    pub fn from_path(path: &Path) -> Result<GitInfo, GitError> {
        use GitError::*;

        let repo = Repository::open(path).or(Err(FailedToOpen))?;
        let head = repo.head().or(Err(FailedToReadHead))?;

        let git_info = GitInfo {
            head_ref: head
                .peel_to_commit()
                .or(Err(FailedToReadHead))?
                .summary()
                .ok_or(FailedToReadHead)?
                .to_string(),

            changes: repo.statuses(None).or(Err(FailedToReadStatus))?.len(),
        };
        Ok(git_info)
    }
}
