/// Represents some content, whether that be
/// a document, an image, a message, a video, or so on.
/// Content can be recursive, so, for example,
/// A Chat might be a set of Messages,
/// and an individual Message could be embedded in a Document.
#[derive(Debug, Clone)]
pub enum Content {
    Root,
    PlainText(String),
}

#[derive(Debug, Clone)]
pub enum Diff {
    PlainText(String),
}
