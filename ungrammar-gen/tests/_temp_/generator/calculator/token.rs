// This file is generated, do not edit

#![allow(clippy::all)]

use diagnostics::FileSpan;

use super::{nodes::*, *};

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Plus(SyntaxToken);
impl std::fmt::Debug for Plus {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { std::fmt::Debug::fmt(&self.0, f) }
}
impl AstToken for Plus {
	fn text(&self) -> Text { unsafe { std::mem::transmute(self.0.text_key().unwrap()) } }
}
impl AstElement for Plus {
	fn can_cast(kind: SyntaxKind) -> bool { kind == SyntaxKind::Plus }

	fn cast(elem: SyntaxElement) -> Option<Self> {
		let tok = elem.into_token()?;
		Self::can_cast(tok.kind()).then(|| Self(tok))
	}

	fn span(&self) -> FileSpan { blanket_impls::default_span(self.0) }

	fn inner(self) -> SyntaxElement { self.0.into() }
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Minus(SyntaxToken);
impl std::fmt::Debug for Minus {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { std::fmt::Debug::fmt(&self.0, f) }
}
impl AstToken for Minus {
	fn text(&self) -> Text { unsafe { std::mem::transmute(self.0.text_key().unwrap()) } }
}
impl AstElement for Minus {
	fn can_cast(kind: SyntaxKind) -> bool { kind == SyntaxKind::Minus }

	fn cast(elem: SyntaxElement) -> Option<Self> {
		let tok = elem.into_token()?;
		Self::can_cast(tok.kind()).then(|| Self(tok))
	}

	fn span(&self) -> FileSpan { blanket_impls::default_span(self.0) }

	fn inner(self) -> SyntaxElement { self.0.into() }
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct IntLit(SyntaxToken);
impl std::fmt::Debug for IntLit {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { std::fmt::Debug::fmt(&self.0, f) }
}
impl AstToken for IntLit {
	fn text(&self) -> Text { unsafe { std::mem::transmute(self.0.text_key().unwrap()) } }
}
impl AstElement for IntLit {
	fn can_cast(kind: SyntaxKind) -> bool { kind == SyntaxKind::IntLit }

	fn cast(elem: SyntaxElement) -> Option<Self> {
		let tok = elem.into_token()?;
		Self::can_cast(tok.kind()).then(|| Self(tok))
	}

	fn span(&self) -> FileSpan { blanket_impls::default_span(self.0) }

	fn inner(self) -> SyntaxElement { self.0.into() }
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct FloatLit(SyntaxToken);
impl std::fmt::Debug for FloatLit {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { std::fmt::Debug::fmt(&self.0, f) }
}
impl AstToken for FloatLit {
	fn text(&self) -> Text { unsafe { std::mem::transmute(self.0.text_key().unwrap()) } }
}
impl AstElement for FloatLit {
	fn can_cast(kind: SyntaxKind) -> bool { kind == SyntaxKind::FloatLit }

	fn cast(elem: SyntaxElement) -> Option<Self> {
		let tok = elem.into_token()?;
		Self::can_cast(tok.kind()).then(|| Self(tok))
	}

	fn span(&self) -> FileSpan { blanket_impls::default_span(self.0) }

	fn inner(self) -> SyntaxElement { self.0.into() }
}
