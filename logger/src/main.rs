use std::{
    collections::HashSet,
    fs::{self, create_dir_all},
    path::PathBuf,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use anyhow::Context;
use clap::Parser;
use client_af::{Client, Product};
use dotenvy::dotenv;
use git2::{Cred, IndexAddOption, PushOptions, RemoteCallbacks, Repository, Signature, Time};
use tokio::time::{interval, MissedTickBehavior};
use tracing::{error, info};

#[derive(Debug, Parser)]
pub struct Args {
    #[clap(long, env)]
    repo: PathBuf,
    #[clap(long, env)]
    email: String,
    #[clap(long, env, hide_env_values = true)]
    password: String,
    #[clap(long, env, default_value = "60")]
    interval_secs: u64,
}

fn overwrite(products: &[Product], repo: &Repository) -> anyhow::Result<()> {
    let mut repo_root = repo.path();
    if !repo.is_bare() {
        repo_root = repo_root.parent().ok_or(anyhow::anyhow!("orphan .git"))?;
    }

    let dir = repo_root.join("vacant");
    create_dir_all(&dir)?;

    let mut to_remove = fs::read_dir(&dir)?
        .map(|res| {
            res.context("process DirEntry")
                .and_then(|entry| entry.path().canonicalize().context("canonicalize"))
        })
        .collect::<Result<HashSet<PathBuf>, _>>()
        .context("collect to_remove")?;

    for product in products {
        let filename = format!("{}-{}", product.product_id, product.area);

        let path = dir.join(filename).with_extension("json");
        std::fs::write(&path, serde_json::to_string_pretty(product)?)?;

        to_remove.remove(&path.canonicalize()?);
    }

    for path in to_remove {
        if path.is_file() {
            std::fs::remove_file(path)?;
        }
    }

    Ok(())
}

fn commit_changes(repo: &Repository) -> anyhow::Result<()> {
    let mut index = repo.index()?;

    index.add_all(["*"].iter(), IndexAddOption::DEFAULT, None)?;
    index.write()?;

    let unix = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs() as i64;
    let signature = Signature::new("logger", "logger@amcoff.net", &Time::new(unix, 0))?;

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

    // Set up callbacks for authentication.
    let mut callbacks = RemoteCallbacks::new();
    callbacks.credentials(|_url, username_from_url, _allowed_types| {
        Cred::ssh_key_from_agent(username_from_url.unwrap())
    });

    // Prepare push options.
    let mut push_options = PushOptions::new();
    push_options.remote_callbacks(callbacks);

    // Push changes to the remote repository.
    let mut remote = repo.find_remote("origin")?;
    remote.push(
        &["refs/heads/main:refs/heads/main"],
        Some(&mut push_options),
    )?;

    info!("pushed");

    Ok(())
}

fn write_and_commit_products(products: &[Product], repo: &Repository) -> anyhow::Result<()> {
    overwrite(products, repo)?;
    commit_changes(repo)
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let _ = dotenv();
    tracing_subscriber::fmt::init();
    let Args { repo, email, password, interval_secs } = Args::parse();

    let repo = Repository::open(repo)?;

    let client = Client::new(email, password);
    let mut interval = interval(Duration::from_secs(interval_secs));
    interval.set_missed_tick_behavior(MissedTickBehavior::Skip);

    loop {
        interval.tick().await;
        match client.list_vacant().await {
            Ok(vacant) => {
                if let Err(e) = write_and_commit_products(&vacant, &repo) {
                    error!("failed to write and commit: {e:?}");
                }
            }
            Err(e) => {
                error!("error listing vacant properties: {e:?}");
            }
        };
    }
}
