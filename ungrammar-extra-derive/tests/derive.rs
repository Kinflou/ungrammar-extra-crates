// External Uses
use ungrammar_extra_derive::SyntaxKind;


#[test]
fn derive() {
    #[derive(SyntaxKind)]
    enum TokenKind {
        #[syntax(lit="string", desc="String")]
        StringLit,
    }

    TokenKind::StringLit {};
}
