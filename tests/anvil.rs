use std::path::Path;

use kitbag::*;

#[test]
fn messing_around() {
    let mut datastore = datastore::Datastore::new(
        Path::new("/Users/slightknack/Desktop/datastore.kit")
    );
    println!("{:#?}", datastore);

    let mut agent = agent::Agent::new("Isaac Clayton");
    datastore.register(&content::Content::Agent(agent.clone()));

    let mut party = namespace::Namespace::new(&mut agent, "Isaac's Party");
    datastore.register(&content::Content::Namespace(party.clone()));

    agent.display = "Isaac Buddy Clayton".to_string();
    datastore.update(&content::Content::Agent(agent.clone()));
    datastore.update(&content::Content::Agent(agent.clone()));

    // let mut welcome = page::Page::child(&mut party, "Welcome", data::Data::N)

    println!("{:#?}", datastore);

    todo!()
}
