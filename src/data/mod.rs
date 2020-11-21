use serde::{Serialize, Deserialize};

pub mod number;
pub mod document;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Data {
    Number(number::Number),
    // Body(document::Document),
}

#[derive(Clone, Serialize, Deserialize)]
pub enum DataDiff {
    NumberDiff(number::NumberDiff)
}
