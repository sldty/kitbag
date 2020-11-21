use serde::{Serialize, Deserialize};

pub mod number;
pub mod document;

// TODO: pub uses

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Data {
    Number(number::Number),
    // Body(document::Document),
}

#[derive(Clone, Serialize, Deserialize)]
pub enum DataDiff {
    NumberDiff(number::NumberDiff)
}
