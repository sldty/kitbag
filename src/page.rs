use serde::{Serialize, Deserialize};
use crate::{
    namespace::Namespace,
    data::Data,
    handle::Identity
};

#[derive(Clone, Serialize, Deserialize)]
pub struct Page {
    namespace: Namespace,
    identity:  Identity, // Page
    parent:    Identity, // Page
    title:     String,
    content:   Data, // Content
    children:  Vec<Identity>, // Page
}

impl Page {
    pub fn identity(&self) -> Identity { self.identity.clone() }

    pub fn context(&self) -> Namespace {
        self.namespace.clone()
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct PageDiff {}

impl PageDiff {
    pub fn make(prev: &Page, next: &Page) -> PageDiff {
        todo!()
    }

    pub fn apply(&self, prev: &Page) -> Page {
        todo!()
    }
}
