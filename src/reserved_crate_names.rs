//! <b style="font-variant:small-caps">reserved_crate_names.csv</b>

use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct Row {
    /// PRIMARY KEY
    pub name: String,
}
