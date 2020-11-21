use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Number(f64);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NumberDiff(f64);
