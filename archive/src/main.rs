use std::{
    collections::HashSet,
    fs::{self, create_dir_all},
    path::PathBuf,
    time::Duration,
};

use anyhow::Context;
use archive_af::{git::{add_commit_push_if_changed, repo_root}, read::queue_history};
use clap::Parser;
use client_af::{Client, Product};
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
    Read(Read),
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

#[derive(Debug, Parser)]
pub struct Read {}

fn overwrite(products: &[Product], repo: &Repository) -> anyhow::Result<()> {
    let dir = repo_root(repo).join("vacant");
    create_dir_all(&dir)?;

    let mut to_remove = fs::read_dir(&dir)?
        .map(|res| {
            res.context("process DirEntry")
                .and_then(|entry| entry.path().canonicalize().context("canonicalize"))
        })
        .collect::<Result<HashSet<PathBuf>, _>>()
        .context("collect to_remove")?;

    for product in products {
        let filename = format!(
            "{}-{}",
            product
                .get("productId")
                .and_then(|v| v.as_str())
                .ok_or(anyhow::anyhow!("missing productId"))?,
            product
                .get("area")
                .and_then(|v| v.as_str())
                .ok_or(anyhow::anyhow!("missing area"))?,
        );

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

fn write_and_commit_products(products: &[Product], repo: &Repository) -> anyhow::Result<()> {
    overwrite(products, repo)?;
    add_commit_push_if_changed(repo)?;
    Ok(())
}

async fn collect(repo: &Repository, args: Collect) {
    let client = Client::new(args.email, args.password);
    let mut interval = interval(Duration::from_secs(args.interval_secs));
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

fn read(repo: &Repository, args: Read) -> anyhow::Result<()> {
    queue_history(repo)?;
    Ok(())
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let _ = dotenv();
    tracing_subscriber::fmt::init();
    let Args { repo, command } = Args::parse();

    let repo = Repository::open(repo)?;

    match command {
        Cmd::Collect(args) => collect(&repo, args).await,
        Cmd::Read(args) => read(&repo, args)?,
    }

    Ok(())
}
