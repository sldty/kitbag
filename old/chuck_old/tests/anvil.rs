use chuck::*;

#[test]
fn document() {
    let account = Account {
        name: "Chuck Norris".to_string(),
        public_key: Box::new(dumb_key::DumbPublic()),
    };
}
