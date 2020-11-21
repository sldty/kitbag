use serde::{Serialize, Deserialize};
use crate::{
    content::Namespace,
    data::Data,
    handle::{Location, Identity}
};

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

    pub fn location(&self) -> Location { self.namespace.find(&self.identity) }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PageDiff {}

impl PageDiff {
    pub fn make(prev: &Page, next: &Page) -> PageDiff {
        todo!()
    }

    pub fn apply(&self, prev: &Page) -> Page {
        todo!()
    }
}
