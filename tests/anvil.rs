use std::path::Path;

use kitbag::*;

#[test]
fn messing_around() {
    let mut datastore = datastore::Datastore::new(
        Path::new("/Users/slightknack/Desktop/datastore.kit")
    );
    println!("{:#?}", datastore);

    let agent = agent::Agent::new("Isaac Clayton");
    let agent_2 = agent::Agent::new("Bob Smith");
    datastore.register(&content::Content::Agent(agent));
    datastore.register(&content::Content::Agent(agent_2));
    println!("{:#?}", datastore);

    todo!()
}
