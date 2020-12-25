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

/// Represents some user-provided data,
/// whether this be a document, a chat room, a video, (and so on).
/// Right now I just have a Number dummy-type.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Data {
    Text(Text),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DataDiff {
    TextDiff(TextDiff)
}

impl Diffable for Data {
    type Diff = DataDiff;

    fn make(&self, next: &Self) -> DataDiff {
        match (self, next) {
            (Data::Text(p), Data::Text(n))
            => DataDiff::TextDiff(Diffable::make(p, n)),
        }
    }

    fn apply(&self, diff: &DataDiff) -> Data {
        match (self, diff) {
            (Data::Text(p), DataDiff::TextDiff(d))
            => Data::Text(Diffable::apply(p, d)),
        }
    }
}
