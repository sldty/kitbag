use serde::{Serialize, Deserialize};
use crate::{
    diff::Diffable,
    data::{Text, TextDiff}
};

// TODO: pub uses
// TODO: figure out an efficient scheme to allow for the following:
// - Versioned, user-defined Data
// - Sandboxed embedded applications through WASM,
// - Translation layers between different data schema.

/// Represents some user-provided `Data`,
/// whether this be a document, a chat room, a video, or otherwise.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Data {
    Text(Text),
}

/// Represents the difference between some `Data.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DataDiff {
    TextDiff(TextDiff)
}

impl Diffable for Data {
    type Diff = DataDiff;

    fn make(prev: &Data, next: &Data) -> DataDiff {
        match (prev, next) {
            (Data::Text(p), Data::Text(n))
            => DataDiff::TextDiff(Diffable::make(p, n)),
        }
    }

    fn apply(prev: &Data, diff: &DataDiff) -> Data {
        match (prev, diff) {
            (Data::Text(p), DataDiff::TextDiff(d))
            => Data::Text(Diffable::apply(p, d)),
        }
    }
}
