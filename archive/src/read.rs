use std::collections::{BTreeMap, HashMap, HashSet};

use git2::Repository;
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};
use tracing::error;

use crate::git::for_each_file;

#[serde_as]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Interesting {
    #[serde_as(as = "DisplayFromStr")]
    product_id: u32,
    #[serde_as(as = "DisplayFromStr")]
    queue_number: u32,
    #[serde(rename = "type")]
    typ: String,
    area: String,
    description: String,
    #[serde_as(as = "DisplayFromStr")]
    sqr_mtrs: f32,
    #[serde_as(as = "Option<DisplayFromStr>")]
    reserved: Option<bool>,
}

#[derive(Debug, Serialize, PartialEq, PartialOrd, Eq, Ord)]
#[serde(rename_all = "camelCase")]
struct Series {
    name: String,
    color: String,
    data: Vec<Option<u32>>,
}

#[derive(Debug, Serialize)]
pub struct Chart {
    categories: Vec<i64>,
    series: Vec<Series>,
}

#[derive(Debug, Deserialize)]
pub struct QueueHistoryQuery {
    max: Option<u32>,
}

pub fn queue_history(repo: &Repository, query: &QueueHistoryQuery) -> Result<Chart, git2::Error> {
    let mut product_ids = HashSet::new();
    let mut data = BTreeMap::<_, HashMap<_, _>>::new();

    for_each_file(repo, |commit, path, content| {
        if !path.starts_with("vacant") {
            return;
        }

        let t = commit.time().seconds() + commit.time().offset_minutes() as i64 * 60;

        let entry = data.entry((t, commit.id())).or_default();

        match serde_json::from_slice::<Interesting>(content) {
            Ok(data) => {
                if data.typ != "Lägenhet" {
                    return;
                }

                product_ids.insert(data.product_id);
                entry.insert(data.product_id, data);
            }
            Err(e) => error!(
                "failed to deserialize {path} (commit: {}): {e}",
                commit.id()
            ),
        }
    })?;

    let mut i = 0;
    let len = data.len();
    data.retain(|&(t, _), _| {
      i+=1;

      t >= 1721253600 && (i % 10 == 0 || len - i < 100)
    });

    let mut series = product_ids
        .iter()
        .filter_map(|&product_id| {
            let v = data
                .values()
                .filter_map(|map| map.get(&product_id))
                .next_back()?;

              if query.max.is_some_and(|max| v.queue_number > max) {
                return None;
              }

            let data = data
                .values()
                .map(|map| map.get(&product_id).map(|i| i.queue_number))
                .collect();

            let mut name = format!("{}; {} ({} kvm)", v.area, v.description, v.sqr_mtrs);
            let reserved = v.reserved.unwrap_or_default();

            if reserved {
                name.push_str(" ✅");
            }

            Some(Series {
                name,
                data,
                color: if reserved { "#f00" } else { "#eee" }.to_owned(),
            })
        })
        .collect::<Vec<_>>();

    series.sort();

    Ok(Chart {
        categories: data.keys().map(|&(k, _)| k).collect(),
        series,
    })
}
