use serde::{Serialize, Deserialize};
use crate::{
    diff::VecDiff,
    content::Namespace,
    data::{DataDiff, Data},
    handle::{Location, Identity},
};

// TODO: linking/backlinking, full text search, etc.
/// Represents a singular `Page` of user-provided `Data`,
/// Whether this be a document, a chat room, a video, and so on.
/// `Page`s can contain sub-pages, the `Data` of a page can embed other pages.
/// So, for instance, you could copy a video's id, and embed it in a document.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Page {
    namespace:     Location, // Namespace
    pub identity:  Identity, // Page
    // None if page is the root page
    parent:        Option<Identity>, // Page
    pub title:     String,
    data:          Data, // Content
    children:      Vec<Identity>, // Page
}

impl Page {
    // TODO: root?

    /// Creates a child `Page` within the context of a `Namespace`.
    pub fn child(&mut self, namespace: &mut Namespace, title: &str, data: Data) -> Page {
        let page = Page {
            namespace: self.namespace.clone(),
            identity:  Identity::new(),
            parent:    Some(self.identity.clone()),
            title:     title.to_string(),
            data,
            children:  vec![],
        };

        self.children.push(page.identity.clone());
        namespace.register_page(&page);
        return page;
    }

    /// Returns the contextual location of the `Page`.
    pub fn location(&self) -> Location { self.namespace.find(&self.identity) }
}

// TODO: call out to Diff to calculate the difference between two Data
/// Represents a difference between two `Pages`.
/// Calculates the underlying difference of the Data it contains.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PageDiff {
    namespace: Option<Location>,
    parent:    Option<Option<Identity>>,
    title:     Option<String>,
    data:      DataDiff,
    children:  VecDiff<Identity>,
}

impl PageDiff {
    /// Finds the difference between two `Pages`s, and creates a `PageDiff`.
    pub fn make(prev: &Page, next: &Page) -> PageDiff {
        todo!()
    }

    // TODO: option types to indicate a conflict?

    /// Applies this diff to another `Page` to create a new `Page`.
    pub fn apply(&self, prev: &Page) -> Page {
        todo!()
    }
}
