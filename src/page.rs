use serde::{Serialize, Deserialize};
use crate::{
    namespace::Namespace,
    content::Content,
    handle::Identity
};

#[derive(Clone, Serialize, Deserialize)]
pub struct Page {
    namespace: Namespace,
    identity:  Identity, // Page
    parent:    Identity, // Page
    title:     String,
    content:   Content, // Content
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
