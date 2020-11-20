use std::collections::HashMap;
use crate::Agent;

// TODO: implement comparing identities,
// e.g. an editor is also a viewer.
#[repr(u8)]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Permission {
    // TODO: Inherit?
    Restricted = 0,
    Viewer,
    Editor,
}

#[derive(Debug, Clone)]
pub struct Permissions {
    collaborators: HashMap<Agent, Permission>,
    base: Permission,
}


impl Permissions {
    pub fn new(base: Permission) -> Permissions {
        Permissions {
            collaborators: HashMap::new(),
            base,
        }
    }

    pub fn permission(&self, agent: &Agent) -> Permission {
        if let Some(permission) = self.collaborators.get(agent) {
            if permission >= &self.base {
                return permission.to_owned();
            }
        }
        return self.base.to_owned();
    }
}
