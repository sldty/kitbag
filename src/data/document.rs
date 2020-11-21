use serde::{Serialize, Deserialize};

#[derive(Clone, Serialize, Deserialize)]
pub enum Heading {
    H1,
    H2,
    H3,
    Caption,
}

// TODO: manual serializer to a markdown-like format.
// TODO: flesh out document workings.
// TODO: implement diffing for documents.

/// Represents a document similar to markdown, but with rich contextual features.
#[derive(Clone, Serialize, Deserialize)]
pub enum Document {
    Heading(Heading, String),
    Text(String),
    Body(Vec<Document>),
}
