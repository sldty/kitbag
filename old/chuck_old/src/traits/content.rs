/// An item of content embedded in a Page.
/// Each page has a single Content.
/// Content may contain multiple more Content.
/// All that can be Address'd is Content.
#[typetag::serde]
pub trait Content {}
