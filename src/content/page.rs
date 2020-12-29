use serde::{Serialize, Deserialize};
use crate::{
    diff::{Diffable, VecDiff},
    content::Namespace,
    data::{DataDiff, Data},
    handle::{Location, Identity},
};
use super::{Contentable, Hierarchy};

// TODO: linking/backlinking, full text search, etc.
/// Represents a singular `Page` of user-provided `Data`,
/// Whether this be a document, a chat room, a video, and so on.
/// `Page`s can contain sub-pages, the `Data` of a page can embed other pages.
/// So, for instance, you could copy a video's id, and embed it in a document.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Page {
    pub hierarchy: Hierarchy<Namespace, ()>,
    pub page_parent: Option<Identity>, // none if root page
    pub page_children: Vec<Identity>,
    pub title: String,
    pub data:  Data, // Content
}

impl Page {
    /// Creates a root `Page` within the context of a `Namespace`.
    pub fn root(namespace: &mut Namespace, title: &str, data: Data) -> Page {
        let page = Page {
            hierarchy:     Hierarchy::new(namespace),
            page_parent:   None,
            page_children: vec![],
            title:         title.to_string(),
            data:          data,
        };

        namespace.hierarchy.register(&page);
        namespace.roots.push(Contentable::identity(&page));

        return page;
    }

    /// Creates a child `Page` within the context of a `Namespace`.
    pub fn child(&mut self, namespace: &mut Namespace, title: &str, data: Data) -> Page {
        let page = Page {
            hierarchy:     Hierarchy::new(namespace),
            page_parent:   Some(Contentable::identity(self)),
            page_children: vec![],
            title:         title.to_string(),
            data:          data,
        };

        namespace.hierarchy.register(&page);
        self.page_children.push(Contentable::identity(self));

        return page;
    }
}

impl Contentable for Page {
    fn context(&self)  -> Location { self.hierarchy.parent.clone()   }
    fn identity(&self) -> Identity { self.hierarchy.identity.clone() }
}

// TODO: call out to Diff to calculate the difference between two Data
/// Represents a difference between two `Pages`.
/// Calculates the underlying difference of the Data it contains.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PageDiff {
    pub hierarchy: Hierarchy<Namespace, ()>,
    pub page_parent: Option<Identity>, // none if root page
    pub page_children: Vec<Identity>,
    pub title: String,
    pub data:  Data, // Content
}

impl Diffable for Page {
    type Diff = PageDiff;

    fn make(prev: &Self, next: &Self) -> Self::Diff {
        todo!()
    }

    fn apply(prev: &Self, diff: &Self::Diff) -> Self {
        todo!()
    }
}
