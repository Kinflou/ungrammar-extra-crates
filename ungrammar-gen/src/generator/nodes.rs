// Standard Uses
use std::path::Path;

// Crate Uses
use super::utils::io::{self, Uses};
use crate::generator::gen::Generator;

// External Uses
use ungrammar_extra::KindsMetaInfo;



impl<'a, M: KindsMetaInfo + 'static> Generator<'a, M> {
	pub(crate) fn gen_ast_traits(&self, out: &Path) {
		io::write_generated(
			out,
			"mod.rs",
			Some(Uses { 
                    mods: vec![
                        "token".to_string(), "kind".to_string(),
                        "ast".to_string(), //"nodes".to_string(), 
                        "blanket_impls".to_string()
                    ],
                    std: vec![],
                    krate: vec!["kind::SyntaxKind".to_string()],
                    external: vec!["text::Text".to_string(), "diagnostics::FileSpan".to_string()],
                
            }),
			indoc::indoc! {r#"
                pub type SyntaxNode = cstree::syntax::SyntaxNode<SyntaxKind>;
                pub type SyntaxToken = cstree::syntax::SyntaxToken<SyntaxKind>;
                pub type SyntaxElement = cstree::syntax::SyntaxElement<SyntaxKind>;
                pub type SyntaxElementRef<'a> = cstree::syntax::SyntaxElementRef<'a, SyntaxKind>;
                pub type SyntaxNodeChildren<'a> = cstree::syntax::SyntaxNodeChildren<'a, SyntaxKind>;

                pub type ResolvedNode = cstree::syntax::ResolvedNode<SyntaxKind>;
                pub type ResolvedToken = cstree::syntax::ResolvedToken<SyntaxKind>;
                pub type ResolvedElement = cstree::syntax::ResolvedElement<SyntaxKind>;

                pub trait AstNode: Sized {}

                pub trait AstToken: Sized {
                    fn text(&self) -> Text;
                }

                pub trait AstElement: Sized {
                    fn can_cast(kind: SyntaxKind) -> bool;

                    fn cast(elem: SyntaxElement) -> Option<Self>;

                    fn span(&self) -> FileSpan;

                    fn inner(self) -> SyntaxElement;
                }

                fn children<'a, T: 'a + AstElement>(node: &'a SyntaxNode) -> impl Iterator<Item = T> + 'a {
                    node.children_with_tokens()
                        .map(|x| match x {
                            SyntaxElementRef::Node(node) => SyntaxElement::Node(node.clone()),
                            SyntaxElementRef::Token(token) => SyntaxElement::Token(token.clone()),
                        })
                        .filter_map(T::cast)
                }

        "#}.to_owned(),
		);
	}
}
