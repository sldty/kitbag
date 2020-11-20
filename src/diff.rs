use serde::{Serialize, Deserialize};

use crate::{
    agent::AgentDiff,
    page::PageDiff,
    namespace::NamespaceDiff,
    storable::Storable
};

#[derive(Clone, Serialize, Deserialize)]
pub enum Diff {
    Agent(AgentDiff),
    Namespace(NamespaceDiff),
    Page(PageDiff),
}

impl Diff {
    fn make(prev: &Storable, next: &Storable) -> Option<Diff> {
        use Storable::*;
        if prev.identity() != next.identity() { return None; }
        let diff: Diff = match (prev, next) {
            ( Agent(p),     Agent(n)     ) => Diff::Agent(AgentDiff::make(p, n)),
            ( Namespace(p), Namespace(n) ) => Diff::Namespace(NamespaceDiff::make(p, n)),
            ( Page(p),      Page(n)      ) => Diff::Page(PageDiff::make(p, n)),
            _ => return None,
        };
        return Some(diff);
    }

    fn apply(tip: &Storable, diff: &Diff) -> Option<Storable> {
        use Storable::*;
        match (tip, diff) {
            ( Agent(s),     Diff::Agent(d)     ) => d.apply(s),
            ( Namespace(s), Diff::Namespace(d) ) => d.apply(s),
            ( Page(s),      Diff::Page(d)      ) => d.apply(s),
            _ => return None,
        }
        todo!()
    }
}
