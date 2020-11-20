use serde::{Serialize, Deserialize};

#[derive(Clone, Serialize, Deserialize)]
pub enum Heading {
    H1,
    H2,
    H3,
    Caption,
}

#[derive(Clone, Serialize, Deserialize)]
pub enum Document {
    Heading(Heading, String),
    Text(String),
    Body(Vec<Document>),
}
