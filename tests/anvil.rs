use std::path::Path;

use kitbag::*;
use kitbag::content::*;

#[test]
fn messing_around() {
    let mut datastore = Datastore::new(
        Path::new("/Users/slightknack/Desktop/datastore.kit")
    );
    println!("{:#?}", datastore);

    let mut agent = Agent::new("Isaac Clayton");
    datastore.register(&Content::Agent(agent.clone()));

    let mut party = Namespace::new(&mut agent, "Isaac's Party");
    datastore.register(&Content::Namespace(party.clone()));

    agent.display = "Isaac Buddy Clayton".to_string();
    datastore.update(&Content::Agent(agent.clone()));
    datastore.update(&Content::Agent(agent.clone()));

    // let mut welcome = Page::child(&mut party, "Welcome", data::Data::N)

    println!("{:#?}", datastore);

    todo!()
}
