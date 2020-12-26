pub trait Contentable {
    fn context(&self)  -> Location;
    fn identity(&self) -> Identity;

    fn location(&self) -> Location {
        Contentable::context(self).find(self.identity())
    }
}

pub struct Hierarchy<A, B> {
    item: A,
    children: HashSet<Identity>,
}

impl<A, B> Hierarchy<A, B> {
    pub fn new(item: A) -> Hierarchy<A, B> {
        Hierarchy {
            item,
            children: HashSet::new()
        }
    }

    pub fn register(&mut self, child: &mut B) {
        self.children.insert(Contentable::indenity(child));
    }

}
