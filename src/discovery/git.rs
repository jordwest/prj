use super::cache::VcsInfo;
use git2::{Repository, StatusOptions};
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

    let statuses = repo
        .statuses(Some(
            StatusOptions::new()
                .include_ignored(false)
                .include_untracked(true),
        ))
        .or(Err(FailedToReadStatus))?;

    let vcs_info = VcsInfo {
        last_commit_summary: head
            .peel_to_commit()
            .or(Err(FailedToReadHead))?
            .summary()
            .ok_or(FailedToReadHead)?
            .to_string(),

        current_branch_name: head.shorthand().ok_or(FailedToReadHead)?.to_string(),

        uncommitted_changes: statuses.len(),
    };
    Ok(vcs_info)
}
