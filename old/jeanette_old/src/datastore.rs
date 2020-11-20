use std::{
    rc::Rc,
    cell::RefCell,
    collections::HashMap,
    path::PathBuf
};
use crate::{
    Identity,
    Page,
    Address,
    Content,
    Name,
    Namespace,
};

pub const DATASTORE_PATH: &str = "~/Desktop/account.kit";

// TODO: address, but for namespaces?
/// A content-addressed storage and caching layer.
/// Changes to pages, namespaces, etc. must be added here.
#[derive(Debug)]
pub struct Datastore {
    cached_names: HashMap<Name, Namespace>,
    cached_idens: HashMap<Identity, Page>,
    cached_addrs: HashMap<Address, Content>,
    path:         PathBuf,
}

impl Datastore {
    pub fn init() -> Datastore {
        // TODO: initialize file system
        Datastore {
            cached_names: HashMap::new(),
            cached_idens: HashMap::new(),
            cached_addrs: HashMap::new(),
            path: PathBuf::from(DATASTORE_PATH),
        }
    }

    pub fn commit_iden(&mut self, page: Page) {
        todo!("committing a page is not yet implemented");
    }

    pub fn commit_addr(&mut self, content: Content) {
        todo!("committing content is not yet implemented");
    }

    pub fn commit_name(&mut self, namespace: Namespace) {
        todo!("committing a namespace is not yet implemented");
    }

    pub fn page(&self, iden: &Identity) -> Page {
        if let Some(page) = self.cached_idens.get(iden) {
            page.clone()
        } else {
            todo!("Still need to implement local storage for pages")
        }
    }

    pub fn content(&self, address: &Address) -> Content {
        if let Some(content) = self.cached_addrs.get(address) {
            content.clone()
        } else {
            todo!("Still need to implement local storage for content")
        }
    }

    pub fn namespace(&self, name: &Name) -> Namespace {
        if let Some(namespace) = self.cached_names.get(name) {
            namespace.clone()
        } else {
            todo!("Still need to implement local storage for namespaces")
        }
    }
}
