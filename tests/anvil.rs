use std::path::Path;

// use kitbag::*;
// use kitbag::content::*;
//
// #[test]
// fn messing_around() {
//     let mut datastore = Datastore::new(
//         Path::new("/Users/slightknack/Desktop/kitbag_datastore")
//     ).unwrap();
//
//     println!("{:#?}", datastore);
//
//     let mut agent = Agent::new("Isaac Clayton");
//     datastore.register(&Content::Agent(agent.clone())).unwrap();
//
//     let mut party = Namespace::new(&mut agent, "Isaac's Party");
//     datastore.register(&Content::Namespace(party.clone())).unwrap();
//
//     agent.display = "Isaac Buddy Clayton".to_string();
//     datastore.update(&Content::Agent(agent.clone())).unwrap();
//
//     let mut welcome = Page::root(
//         &mut party,
//         "Welcome",
//         data::Data::Text(data::Text::new(String::from("Hello!")),
//     ));
//     datastore.register(&Content::Page(welcome.clone())).unwrap();
//
//     println!("{:#?}", datastore);
//
//     todo!()
// }
