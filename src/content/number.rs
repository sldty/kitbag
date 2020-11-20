use serde::{Serialize, Deserialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct Number(f64);

#[derive(Clone, Serialize, Deserialize)]
pub struct NumberDiff(f64);
