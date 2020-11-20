use serde::{Serialize, Deserialize};
use crate::traits::Storable;

pub mod number;
pub mod document;

// TODO: enum?
// #[typetag::serde]
// pub trait Content: Storable {
//
// }

#[derive(Clone, Serialize, Deserialize)]
pub enum Content {
    Number(number::Number),
    // Body(document::Document),
}

#[derive(Clone, Serialize, Deserialize)]
pub enum ContentDiff {
    NumberDiff(number::NumberDiff)
}
