// External Uses
use logos::Logos;
use ungrammar_extra::KindsMetaInfo;
use ungrammar_extra_derive::SyntaxKind;


#[test]
fn logos_derived() {
    #[derive(SyntaxKind, Logos)]
    enum TokenKind {
        #[syntax(lit="bool", desc="boolean")]
        #[regex("true|false")]
        BoolLit,

        #[syntax(lit="+")]
        #[token("+")]
        Plus,
    }

    // Creating a variant and asserting literals means it exists so the macro is likely working
    SyntaxKind::BoolLit {};
    let bool_lit = *SyntaxKind::literals().first().unwrap();
    assert_eq!("bool", bool_lit);
    assert_eq!("BoolLit", *SyntaxKind::kinds().get(bool_lit).unwrap());
    assert_eq!("boolean", *SyntaxKind::descriptions().get(bool_lit).unwrap());


    SyntaxKind::Plus {};
    let plus_lit = *SyntaxKind::literals().get(1).unwrap();
    assert_eq!("+", plus_lit);
    assert_eq!("Plus", *SyntaxKind::kinds().get(plus_lit).unwrap());
    assert_eq!("Plus", *SyntaxKind::descriptions().get(plus_lit).unwrap());

}
