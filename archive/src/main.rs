use std::{
    collections::HashSet,
    fs::{self, create_dir_all},
    path::PathBuf,
    time::Duration,
};

use anyhow::Context;
use archive_af::git::{add_commit_push_if_changed, repo_root};
use clap::Parser;
use client_af::{Client, Credentials, Property};
use dotenvy::dotenv;
use git2::Repository;
use tokio::time::{interval, MissedTickBehavior};
use tracing::error;

#[derive(Debug, Parser)]
pub struct Args {
    #[clap(long, env)]
    repo: PathBuf,
    #[clap(subcommand)]
    command: Cmd,
}

#[derive(Debug, Parser)]
pub enum Cmd {
    Collect(Collect),
}

#[derive(Debug, Parser)]
pub struct Collect {
    #[clap(long, env)]
    email: String,
    #[clap(long, env, hide_env_values = true)]
    password: String,
    #[clap(long, env, default_value = "60")]
    interval_secs: u64,
}

fn overwrite(properties: &[Property], repo: &Repository) -> anyhow::Result<()> {
    let dir = repo_root(repo).join("vacant");
    create_dir_all(&dir)?;

    let mut to_remove = fs::read_dir(&dir)?
        .map(|res| {
            res.context("process DirEntry")
                .and_then(|entry| entry.path().canonicalize().context("canonicalize"))
        })
        .collect::<Result<HashSet<PathBuf>, _>>()
        .context("collect to_remove")?;

    for property in properties {
        let filename = format!("{}-{}", property.id, property.area,);

        let path = dir.join(filename).with_extension("json");
        std::fs::write(&path, serde_json::to_string_pretty(property)?)?;

        to_remove.remove(&path.canonicalize()?);
    }

    for path in to_remove {
        if path.is_file() {
            std::fs::remove_file(path)?;
        }
    }

    Ok(())
}

fn write_and_commit_properties(properties: &[Property], repo: &Repository) -> anyhow::Result<()> {
    overwrite(properties, repo)?;
    add_commit_push_if_changed(repo)?;
    Ok(())
}

async fn collect(repo: &Repository, args: Collect) {
    let client = Client::new().with_credentials(Credentials::new(args.email, args.password));
    let mut interval = interval(Duration::from_secs(args.interval_secs));
    interval.set_missed_tick_behavior(MissedTickBehavior::Skip);

    loop {
        interval.tick().await;
        match client.list_vacant().await {
            Ok(vacant) => {
                if let Err(e) = write_and_commit_properties(&vacant, repo) {
                    error!("failed to write and commit: {e:?}");
                }
            }
            Err(e) => {
                error!("error listing vacant properties: {e:?}");
            }
        };
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let _ = dotenv();
    tracing_subscriber::fmt::init();
    let Args { repo, command } = Args::parse();

    let repo = Repository::open(repo)?;

    match command {
        Cmd::Collect(args) => collect(&repo, args).await,
    }

    Ok(())
}
