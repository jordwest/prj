use super::cache::VcsInfo;
use git2::Repository;
use std::path::Path;

#[derive(Debug)]
pub enum GitError {
    FailedToOpen,
    FailedToReadHead,
    FailedToReadStatus,
}

pub fn fetch_vcs_info(path: &Path) -> Result<VcsInfo, GitError> {
    use GitError::*;

    let repo = Repository::open(path).or(Err(FailedToOpen))?;
    let head = repo.head().or(Err(FailedToReadHead))?;

    let vcs_info = VcsInfo {
        last_commit_summary: head
            .peel_to_commit()
            .or(Err(FailedToReadHead))?
            .summary()
            .ok_or(FailedToReadHead)?
            .to_string(),

        current_branch_name: head.shorthand().ok_or(FailedToReadHead)?.to_string(),

        uncommitted_changes: repo.statuses(None).or(Err(FailedToReadStatus))?.len(),
    };
    Ok(vcs_info)
}
