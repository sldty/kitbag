use crate::{
    Permission,
    Permissions,
    Identity,
    Agent,
    Name, Page, Datastore, Content,
};

#[derive(Debug, Clone)]
pub struct Namespace {
    pub name: Name,
    owner:    Agent,
    permissions: Permissions,
    /// Root children pages.
    pub root: Page,
}

impl Namespace {
    pub fn new(owner: &Agent, datastore: Datastore) -> Namespace {
        let name = Name::new(owner);

        Namespace {
            owner: owner.clone(),
            name:  name.clone(),
            root:  Page::root(&name, &owner.name, Content::Root).id,
            permissions: Permissions::new(Permission::Restricted),
        }
    }

    pub fn permission(&self, agent: &Agent) -> Permission {
        if agent == &self.owner { return Permission::Editor };
        self.permissions.permission(agent)
    }
}

impl PartialEq for Namespace {
    fn eq(&self, other: &Self) -> bool {
         self.owner == other.owner
        && self.name == other.name
    }
}
impl Eq for Namespace {}
