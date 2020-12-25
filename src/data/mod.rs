use serde::{Serialize, Deserialize};

pub mod number;
// pub mod document;
pub mod plain_text;

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
    Number(number::Number),
    // Body(document::Document),
    PlainText(plain_text::PlainText),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DataDiff {
    NumberDiff(number::NumberDiff)
}
