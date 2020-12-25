use serde::{Serialize, Deserialize};
use crate::diff::VecDiff;
use crate::diff::Diffable;

// A plain text file, like some code.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Text(String);

/// Diff runs on lines.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextDiff(VecDiff<String>);

fn lines_inclusive(string: &str) -> Vec<String> {
    string.split("\n").map(|x| x.to_string() + "\n").collect()
}

impl Diffable for Text {
    type Diff = TextDiff;

    fn make(&self, next: &Text) -> TextDiff {
        TextDiff(VecDiff::make(
            &lines_inclusive(&self.0),
            &lines_inclusive(&next.0),
        ))
    }

    fn apply(&self, diff: &TextDiff) -> Text {
        Text(diff.0.apply(&lines_inclusive(&self.0)).join(""))
    }
}
