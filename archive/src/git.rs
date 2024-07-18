use std::{
    collections::HashSet,
    path::Path,
    process::{Command, Output},
    time::{SystemTime, UNIX_EPOCH},
};

use git2::{Commit, IndexAddOption, Repository, Signature, Time, TreeWalkMode, TreeWalkResult};
use tracing::{error, info, warn};

pub fn repo_root(repo: &Repository) -> &Path {
    let path = repo.path();
    if repo.is_bare() {
        path
    } else {
        path.parent().unwrap()
    }
}

pub fn add_commit_push_if_changed(repo: &Repository) -> Result<(), git2::Error> {
    let mut index = repo.index()?;

    index.add_all(["*"].iter(), IndexAddOption::DEFAULT, None)?;
    index.write()?;

    let tree_id = index.write_tree()?;
    let tree = repo.find_tree(tree_id)?;

    // Get the current HEAD commit to use as the parent of the new commit.
    let head = repo.head()?;
    let head_commit = head.peel_to_commit()?;
    let head_tree = head_commit.tree()?;

    // Create a diff between the index and the current HEAD commit's tree.
    let diff = repo.diff_tree_to_tree(Some(&head_tree), Some(&tree), None)?;

    // Check if there are any differences.
    if diff.deltas().len() == 0 {
        info!("no changes to commit");
        return Ok(());
    }

    let unix = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("are the 1970s really in the future?")
        .as_secs() as i64;
    let signature = Signature::new("logger", "logger@amcoff.net", &Time::new(unix, 0))?;

    let parent_commit = repo.head()?.peel_to_commit()?;

    repo.commit(
        Some("HEAD"),
        &signature,
        &signature,
        "Update",
        &tree,
        &[&parent_commit],
    )?;

    let stats = diff.stats()?;

    info!(
        files_changed = stats.files_changed(),
        insertions = stats.insertions(),
        deletions = stats.deletions(),
        "committed changes"
    );

    match push(repo) {
        Ok(output) if output.status.success() => {
            info!("pushed");
        }
        Ok(output) => {
            warn!(
                "git push exited with status {}: {:#?}",
                output.status, output.stderr
            );
        }
        Err(e) => {
            error!("failed to push: {e}");
        }
    }

    Ok(())
}

pub fn push(repo: &Repository) -> std::io::Result<Output> {
    Command::new("git")
        .arg("push")
        .current_dir(repo_root(repo))
        .output()
}

pub fn pull(repo: &Repository) -> std::io::Result<Output> {
    Command::new("git")
        .arg("pull")
        .current_dir(repo_root(repo))
        .output()
}

pub fn for_each_file(
    repo: &Repository,
    mut entry_cb: impl FnMut(&Commit, String, &[u8]),
) -> Result<(), git2::Error> {
    // Set to track processed commits
    let mut processed_commits = HashSet::new();

    // Traverse the commits
    let mut revwalk = repo.revwalk()?;
    revwalk.push_head()?;

    for oid in revwalk {
        let oid = oid?;
        if processed_commits.contains(&oid) {
            continue;
        }

        let commit = repo.find_commit(oid)?;
        processed_commits.insert(oid);

        // Get the trees for the current commit and its parent (if any)
        let tree = commit.tree()?;

        // Read each file in the current commit
        tree.walk(TreeWalkMode::PreOrder, |root, entry| {
            if let Some(blob) = entry
                .to_object(repo)
                .ok()
                .and_then(|obj| obj.into_blob().ok())
            {
                let file_path = format!("{}{}", root, entry.name().unwrap_or(""));

                entry_cb(&commit, file_path, blob.content())
            }
            TreeWalkResult::Ok
        })?;
    }

    Ok(())
}
