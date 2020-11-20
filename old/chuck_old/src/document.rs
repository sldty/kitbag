use serde::{Serialize, Deserialize};
use crate::traits::{
    Storable,
    Content,
};

#[derive(Clone, Serialize, Deserialize)]
pub struct Document {
    title: String,
    content: String,
}

#[typetag::serde] impl Content for Document {}
#[typetag::serde] impl Storable for Document {}
