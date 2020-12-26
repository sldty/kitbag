use std::{
    marker::PhantomData,
    collections::HashSet,
};

use serde::{Serialize, Deserialize};

use crate::{
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
    pub parent:      Identity,
    pub children:    HashSet<Identity>,
    _phantom_parent: PhantomData<A>,
    _phantom_child:  PhantomData<B>,
}

impl<A, B> Hierarchy<A, B> where
    A: Contentable,
    B: Contentable,
{
    pub fn new(parent: A) -> Hierarchy<A, B> {
        Hierarchy {
            parent:          Contentable::identity(&parent),
            children:        HashSet::new(),
            _phantom_parent: PhantomData,
            _phantom_child:  PhantomData,
        }
    }

    pub fn register(&mut self, child: &mut B) {
        self.children.insert(Contentable::identity(child));
    }
}
