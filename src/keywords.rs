//! <b style="font-variant:small-caps">keywords.csv</b>

use chrono::{DateTime, Utc};
use serde_derive::{Deserialize, Serialize};
use std::borrow::Borrow;
use std::cmp::Ordering;
use std::hash::{Hash, Hasher};

/// Primary key of **keywords.csv**.
#[derive(Serialize, Deserialize, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
#[serde(transparent)]
#[repr(transparent)]
pub struct KeywordId(pub u32);

/// One row of **keywords.csv**.
#[derive(Deserialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct Row {
    /// PRIMARY KEY
    pub id: KeywordId,
    /// UNIQUE
    pub keyword: String,
    pub crates_cnt: u32,
    #[serde(deserialize_with = "crate::datetime::de")]
    pub created_at: DateTime<Utc>,
}

impl Ord for Row {
    fn cmp(&self, other: &Self) -> Ordering {
        KeywordId::cmp(&self.id, &other.id)
    }
}

impl PartialOrd for Row {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        KeywordId::partial_cmp(&self.id, &other.id)
    }
}

impl Eq for Row {}

impl PartialEq for Row {
    fn eq(&self, other: &Self) -> bool {
        KeywordId::eq(&self.id, &other.id)
    }
}

impl Hash for Row {
    fn hash<H: Hasher>(&self, state: &mut H) {
        KeywordId::hash(&self.id, state);
    }
}

impl Borrow<KeywordId> for Row {
    fn borrow(&self) -> &KeywordId {
        &self.id
    }
}
