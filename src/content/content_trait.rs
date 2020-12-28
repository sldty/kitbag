use std::{
    marker::PhantomData,
    collections::HashSet,
};

use serde::{Serialize, Deserialize};
use crate::{
    diff::{Diffable, SetDiff, Atom},
    handle::{Identity, Location}
};

pub trait Contentable {
    fn context(&self)  -> Location;
    fn identity(&self) -> Identity;

    fn location(&self) -> Location {
        Contentable::context(self).cd(&Contentable::identity(self))
    }
}

impl Contentable for () {
    fn context(&self) -> Location {
        Location::root()
    }

    fn identity(&self) -> Identity {
        unreachable!()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Hierarchy<A, B> where
    A: Contentable,
    B: Contentable,
{
    pub parent:     Location,
    pub identity:   Identity,
    pub children:   HashSet<Identity>,
    phantom_parent: PhantomData<A>,
    phantom_child:  PhantomData<B>,
}

impl<A, B> Hierarchy<A, B> where
    A: Contentable,
    B: Contentable,
{
    pub fn new(parent: &A) -> Hierarchy<A, B> {
        Hierarchy {
            parent:         Contentable::location(parent),
            identity:       Identity::new(),
            children:       HashSet::new(),
            phantom_parent: PhantomData,
            phantom_child:  PhantomData,
        }
    }

    pub fn register(&mut self, child: &B) {
        self.children.insert(Contentable::identity(child));
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HierarchyDiff<A, B> {
    parent:     Option<Location>,
    children:   SetDiff<Identity>,
    phantom_parent: PhantomData<A>,
    phantom_child:  PhantomData<B>,
}

impl<A, B> Diffable for Hierarchy<A, B> where
    A: Contentable,
    B: Contentable,
{
    type Diff = HierarchyDiff<A, B>;

    fn make(prev: &Hierarchy<A, B>, next: &Hierarchy<A, B>) -> HierarchyDiff<A, B> {
        HierarchyDiff {
            parent: Diffable::make(
                &Atom::new(&prev.parent),
                &Atom::new(&next.parent),
            ),
            children:       Diffable::make(&prev.children, &next.children),
            phantom_parent: PhantomData,
            phantom_child:  PhantomData,
        }
    }

    fn apply(prev: &Hierarchy<A, B>, diff: &HierarchyDiff<A, B>) -> Hierarchy<A, B> {
        Hierarchy {
            parent: Diffable::apply(
                &Atom::new(&prev.parent),
                &diff.parent,
            ).into_inner(),
            identity:       prev.identity.clone(),
            children:       Diffable::apply(&prev.children, &diff.children),
            phantom_parent: PhantomData,
            phantom_child:  PhantomData,
        }
    }
}
