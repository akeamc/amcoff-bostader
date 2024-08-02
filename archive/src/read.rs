use std::collections::{BTreeMap, HashMap, HashSet};

use afbostader::{Property, PropertyType};
use git2::Repository;
use serde::{Deserialize, Serialize};
use tracing::error;

use crate::git::for_each_file;

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

#[derive(Debug, Deserialize, Default)]
pub struct QueueHistoryQuery {
    max: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ArchiveEntry {
    V010(afbostader::Product),
    V011(afbostader::Property),
}

impl From<ArchiveEntry> for Property {
    fn from(value: ArchiveEntry) -> Self {
        match value {
            ArchiveEntry::V010(product) => product.into(),
            ArchiveEntry::V011(property) => property,
        }
    }
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

        match serde_json::from_slice::<ArchiveEntry>(content).map(Property::from) {
            Ok(data) => {
                if data.property_type != PropertyType::Apartment {
                    return;
                }

                product_ids.insert(data.id);
                entry.insert(data.id, data);
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
        i += 1;

        t >= 1721253600 && (i % 50 == 0 || len - i < 100)
    });

    let mut series = product_ids
        .iter()
        .filter_map(|&product_id| {
            let v = data
                .values()
                .filter_map(|map| map.get(&product_id))
                .next_back()?;

            if query
                .max
                .is_some_and(|max| v.queue_position.position.unwrap() > max)
            {
                return None;
            }

            let data = data
                .values()
                .map(|map| {
                    map.get(&product_id)
                        .map(|i| i.queue_position.position.unwrap())
                })
                .collect();

            let mut name = format!("{}; {} ({} kvm)", v.area, v.description, v.size_sqm);

            if v.reserved {
                name.push_str(" ✅");
            }

            Some(Series {
                name,
                data,
                color: if v.reserved { "#ff000080" } else { "#00000020" }.to_owned(),
            })
        })
        .collect::<Vec<_>>();

    series.sort();

    Ok(Chart {
        categories: data.keys().map(|&(k, _)| k).collect(),
        series,
    })
}
