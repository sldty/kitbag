use crate::{tag, Agent};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Name {
    pub agent: Agent,
    pub tag:   Vec<u8>,
}

impl Name {
    pub fn new(agent: &Agent) -> Name {
        Name {
            agent: agent.clone(),
            tag:   tag(),
        }
    }
}
