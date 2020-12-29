use serde::{Serialize, Deserialize};

use crate::content::{
    AgentDiff,
    PageDiff,
    NamespaceDiff,
    Content,
};
use crate::diff::Diffable;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Diff {
    Agent(AgentDiff),
    Namespace(NamespaceDiff),
    Page(PageDiff),
}

impl Diff {
    pub fn make(prev: &Content, next: &Content) -> Option<Diff> {
        use Content::*;
        let diff: Diff = match (prev, next) {
            (Agent(p),     Agent(n)    ) => Diff::Agent(Diffable::make(p, n)),
            (Namespace(p), Namespace(n)) => Diff::Namespace(Diffable::make(p, n)),
            (Page(p),      Page(n)     ) => Diff::Page(Diffable::make(p, n)),
            _ => return None,
        };
        return Some(diff);
    }

    pub fn apply(tip: &Content, diff: &Diff) -> Option<Content> {
        use Content::*;
        let applied: Content = match (tip, diff) {
            (Agent(s),     Diff::Agent(d)    ) => Agent(Diffable::apply(s, d)),
            (Namespace(s), Diff::Namespace(d)) => Namespace(Diffable::apply(s, d)),
            (Page(s),      Diff::Page(d)     ) => Page(Diffable::apply(s, d)),
            _ => return None,
        };
        return Some(applied);
    }
}
