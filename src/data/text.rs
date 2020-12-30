use serde::{Serialize, Deserialize};
use crate::diff::VecDiff;
use crate::diff::{Lines, Diffable};

/// A plain text file, like some code.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Text(String);

impl Text {
    pub fn new(string: String) -> Text { Text(string) }
}

/// The difference of a pair of plain text files.
/// Difference is run on the lines in the file.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextDiff(VecDiff<String>);

impl Diffable for Text {
    type Diff = TextDiff;

    fn make(prev: &Text, next: &Text) -> TextDiff {
        TextDiff(Diffable::make(
            &Lines::new(prev.0.to_string()),
            &Lines::new(next.0.to_string()),
        ))
    }

    fn apply(prev: &Text, diff: &TextDiff) -> Text {
        Text(Diffable::apply(
            &Lines::new(prev.0.to_string()).lines_inclusive(),
            &diff.0,
        ).join(""))
    }
}
