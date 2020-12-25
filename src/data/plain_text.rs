use serde::{Serialize, Deserialize};
use crate::diff::VecDiff;

// A plain text file, like some code
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlainText(String);

/// Diff runs on lines
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlainTextDiff(VecDiff<Vec<String>>);
