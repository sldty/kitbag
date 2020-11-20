use serde::{Serialize, Deserialize};
use crate::traits::{Storable, Diff};

#[derive(Clone, Serialize, Deserialize)]
pub struct Number(f64);

#[derive(Clone, Serialize, Deserialize)]
pub struct NumberDiff(f64);
