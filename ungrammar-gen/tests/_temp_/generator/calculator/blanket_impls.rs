// Standard Uses

// External Uses
use cstree::syntax::SyntaxToken;
use diagnostics::FileSpan;

pub fn default_span<K: cstree::Syntax>(token: SyntaxToken<K>) -> FileSpan {
	let range = token.text_range();
	FileSpan {
		start: range.start().into(),
		end: range.end().into(),
		relative: (),
	}
}
