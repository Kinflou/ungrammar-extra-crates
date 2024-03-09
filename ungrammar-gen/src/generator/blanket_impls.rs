// Standard Uses
use std::path::Path;

// Crate Uses
use super::{
    gen::Generator,
    utils::io
};

// External Uses
use ungrammar_extra::KindsMetaInfo;




impl<'a, M: KindsMetaInfo + 'static> Generator<'a, M> {
	pub(crate) fn gen_blanket_impls(&self, out: &Path) {
		io::write_generated(
			out,
			"blanket_impls.rs",
			Some(io::Uses { 
                    mods: vec![],
                    std: vec![],
                    krate: vec![
                        //"super::kind::SyntaxKind".to_string(),
                        "super::SyntaxToken".to_string()
                    ],
                    external: vec![
                        //"text::Text".to_string(),
                        "diagnostics::FileSpan".to_string(),
                    ],
                
            }),
			indoc::indoc! {r#"
                pub fn default_span(token: &SyntaxToken) -> FileSpan {
                    let range = token.text_range();
                    FileSpan {
                        start: range.start().into(),
                        end: range.end().into(),
                        relative: (),
                    }
                }
            "#}.to_owned(),
		);
	}
}

