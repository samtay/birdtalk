use std::fmt::{self, Display};

use chrono::NaiveDate;

use crate::{
    bird::{Bird, BirdPack},
    supabase::Result,
    utils,
};

/// This type is used during a play session, which could be an ad-hoc list of birds selected to
/// review rather than a db-defined [`BirdPack`].
///
/// Note we maintain an internal invariant that the pack identifier matches the birds here,
/// so equality just compares equality of identifiers.
#[derive(Debug, Clone)]
pub struct Pack {
    /// An identifier for the set of birds to play. This could be an actual BirdPack id, a pack of
    /// the day date, or an ad-hoc list of bird ids.
    pub id: PackIdentifier,
    /// The actual birds to play. This should always match the sibling identifier.
    pub birds: Vec<Bird>,
    /// If this is not an ad-hoc list of birds, this is the id of the birdpack in the database. We
    /// store this so we can record stats per birdpack after completion.
    pub birdpack_id: Option<u64>,
}

impl PartialEq for Pack {
    fn eq(&self, other: &Self) -> bool {
        match (self.birdpack_id, other.birdpack_id) {
            // Allow equality even if our pack identifier is different (e.g. Id vs Date)
            (Some(id1), Some(id2)) => id1 == id2,
            _ => self.id == other.id,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum PackIdentifier {
    Id(u64),
    Date(NaiveDate),
    Birds(Vec<u64>),
}

impl Default for PackIdentifier {
    fn default() -> Self {
        PackIdentifier::Date(chrono::offset::Local::now().date_naive())
    }
}

impl From<BirdPack> for Pack {
    fn from(pack: BirdPack) -> Self {
        let id = match pack.day {
            Some(day) => PackIdentifier::Date(day),
            None => PackIdentifier::Id(pack.id),
        };
        Self {
            id,
            birds: pack.birds,
            birdpack_id: Some(pack.id),
        }
    }
}

impl Pack {
    pub async fn fetch_by_id(id: &PackIdentifier) -> Result<Self> {
        match id {
            PackIdentifier::Id(pid) => BirdPack::fetch_by_id(*pid).await.map(|p| Pack {
                id: id.clone(),
                ..p.into()
            }),
            PackIdentifier::Date(day) => BirdPack::fetch_by_day(*day).await.map(|p| Pack {
                id: id.clone(),
                ..p.into()
            }),
            PackIdentifier::Birds(bids) => {
                Bird::fetch_by_ids(bids.iter().copied())
                    .await
                    .map(|birds| Pack {
                        id: id.clone(),
                        birds,
                        birdpack_id: None,
                    })
            }
        }
    }
}

const LIST_DELIM: char = '.';

impl From<&str> for PackIdentifier {
    fn from(query: &str) -> Self {
        let id = query.parse().ok().map(PackIdentifier::Id);
        let date = NaiveDate::parse_from_str(query, "%Y-%m-%d")
            .ok()
            .map(PackIdentifier::Date);
        let birds = query
            .split(LIST_DELIM)
            .map(|s| s.parse().ok())
            .collect::<Option<Vec<u64>>>()
            .filter(|ids| {
                let mut ids = ids.clone();
                ids.sort();
                ids.dedup();
                ids.len() >= 10
            })
            .map(PackIdentifier::Birds);
        id.or(date).or(birds).unwrap_or_else(|| {
            if !query.is_empty() {
                tracing::error!("Failed to parse pack identifier from query: {query}");
                tracing::info!("Defaulting to pack of the day");
            }
            Self::default()
        })
    }
}

impl Display for PackIdentifier {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            PackIdentifier::Id(id) => write!(f, "{id}"),
            PackIdentifier::Date(date) => write!(f, "{date}"),
            PackIdentifier::Birds(birds) => {
                write!(f, "{}", utils::join(birds, LIST_DELIM))
            }
        }
    }
}
