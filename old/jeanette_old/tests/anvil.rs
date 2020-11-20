use jeanette::*;

#[test]
fn messing_around() {
    let account = Agent::new("Isaac Clayton");
    println!("{}", account.name);

    let namespace = Namespace::new(&account);
    println!("Namespace Name: {:#?}", namespace);

    panic!()
}
