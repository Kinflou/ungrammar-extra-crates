// Standard Uses
use std::collections::HashMap;

// External Uses
use ungrammar_extra::KindsMetaInfo;


#[test]
fn literal_of_kind_name() {
    pub enum TokenKind {}

    static TOKENKIND_KINDS: once_cell::sync::Lazy<HashMap<&'static str, &'static str>> = once_cell::sync::Lazy::new(||
        HashMap::from([
          ("foo", "Foo")
        ])
    );
    static TOKENKIND_DESCRIPTIONS: once_cell::sync::Lazy<HashMap<&'static str, &'static str>> = once_cell::sync::Lazy::new(||
        HashMap::from([
            ("foo", "Foo")
        ])
    );


    impl KindsMetaInfo for TokenKind {
        fn literals() -> &'static [&'static str] {&[
            "foo"
        ]}

        fn kinds() -> &'static HashMap<&'static str, &'static str> { &*TOKENKIND_KINDS }

        fn descriptions() -> &'static HashMap<&'static str, &'static str> { &*TOKENKIND_DESCRIPTIONS }
    }

    let first = TokenKind::literals().first().unwrap();
    let literal = TokenKind::kinds().get(first).unwrap();
    let description = TokenKind::kinds().get(first).unwrap();

    assert_eq!(*first, "foo");
    assert_eq!(*literal, "Foo");
    assert_eq!(*description, "Foo");
}

#[test]
fn literal_of_kind() {
    pub enum TokenKind {}

}
