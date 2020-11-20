use serde::{Serialize, Deserialize};
use crate::{
    namespace::Namespace,
    traits::{Storable, Identity, Diff},
    content::Content
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

#[typetag::serde]
impl Storable for Page {
    fn identity(&self) -> Identity { self.identity.clone() }

    fn context(&self) -> Option<Box<dyn Storable>> {
        Some(Box::new(self.namespace.clone()))
    }

    fn find(&self, identity: &Identity) -> Option<Box<dyn Storable>> { None }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct PageDiff {}

#[typetag::serde]
impl Diff for PageDiff {
    fn base(initial: &dyn Storable) -> Box<dyn Diff> where Self: Sized {
        let page: Page = initial.try_into();
    }
    fn new(
        prev: &dyn Storable,
        next: &dyn Storable,
    ) -> Self where Self: Sized {
        todo!()
    }
    fn apply(
        prev: &dyn Storable,
        diff: &dyn Storable,
    ) -> Self where Self: Sized {
        todo!()
    }
}
