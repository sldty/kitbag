use crate::{
    Namespace,
    Identity,
    Version,
    Permissions,
    Content,
    Agent,
    Permission, Datastore, Name,
};

#[derive(Debug, Clone)]
pub struct Page {
    /// None if parent is root.
    pub parent:   Option<Identity>,
    pub id:       Identity,
    permissions:  Permissions,
    pub title:    String,
    pub version:  Version,
    pub children: Vec<Identity>,
}

impl Page {
    pub fn root(
        name: &Name,
        title: &str,
        content: Content,
    ) -> Page {
        let initial_verison = Version::Base(content);

        Page {
            parent:      None,
            id:          Identity::new(&name),
            permissions: todo!(),
            title:       title.to_string(),
            version:     initial_verison,
            children:    vec![],
        }
    }

    pub fn child(
        &mut self,
        title: &str,
        content: Content,
    ) -> Page {
        let initial_verison = Version::Base(content);

        Page {
            parent:      Some(self.id.clone()),
            id:          self.id.new_same_name(),
            // TODO: think about how permissions should work.
            // Ideally work their way up the tree.
            permissions: self.permissions.clone(),
            title:       title.to_string(),
            version:     initial_verison,
            children:    vec![],
        }
    }

    // TODO: think through more fully.
    // i.e. a private document in an otherwise public namespace does not work atm.
    pub fn permission(&self, datastore: Datastore, agent: Agent) -> Permission {
        let own_perm = self.permissions.permission(&agent);
        let ns_perm = datastore.namespace(&self.id.name).permission(&agent);
        return if own_perm >= ns_perm { own_perm } else { ns_perm };
    }
}
