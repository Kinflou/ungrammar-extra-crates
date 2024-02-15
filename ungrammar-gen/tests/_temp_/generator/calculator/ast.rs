// This file is generated, do not edit

#![allow(clippy::all)]

use diagnostics::FileSpan;

use super::{blanket_impls, nodes::*, token::*, *};

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Expr(SyntaxNode);
impl std::fmt::Debug for Expr {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { std::fmt::Debug::fmt(&self.0, f) }
}
impl AstNode for Expr {}
impl AstElement for Expr {
	fn can_cast(kind: SyntaxKind) -> bool { kind == SyntaxKind::Expr }

	fn cast(elem: SyntaxElement) -> Option<Self> {
		let node = elem.into_node()?;
		Self::can_cast(node.kind()).then(|| Self(node))
	}

	fn span(&self) -> FileSpan {
		let range = self.0.text_range();
		FileSpan {
			start: range.start().into(),
			end: range.end().into(),
			relative: (),
		}
	}

	fn inner(self) -> SyntaxElement { self.0.into() }
}
impl Expr {
	pub fn lhs(&self) -> Option<Literal> { children(&self.0).nth(0usize) }

	pub fn op(&self) -> Option<InfixOperator> { children(&self.0).nth(0usize) }

	pub fn rhs(&self) -> Option<Literal> { children(&self.0).nth(1usize) }
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub enum Literal {
	IntLit(IntLit),
	FloatLit(FloatLit),
}
impl std::fmt::Debug for Literal {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::IntLit(x) => std::fmt::Debug::fmt(x, f),
			Self::FloatLit(x) => std::fmt::Debug::fmt(x, f),
		}
	}
}
impl AstNode for Literal {}
impl AstElement for Literal {
	fn can_cast(kind: SyntaxKind) -> bool { matches!(kind, |SyntaxKind::IntLit| SyntaxKind::FloatLit) }

	fn cast(elem: SyntaxElement) -> Option<Self> {
		match elem.kind() {
			SyntaxKind::IntLit => AstElement::cast(elem.clone()).map(Self::IntLit),
			SyntaxKind::FloatLit => AstElement::cast(elem.clone()).map(Self::FloatLit),
			_ => None,
		}
	}

	fn span(&self) -> FileSpan {
		match self {
			Self::IntLit(x) => x.span(),
			Self::FloatLit(x) => x.span(),
		}
	}

	fn inner(self) -> SyntaxElement {
		match self {
			Self::IntLit(x) => x.inner(),
			Self::FloatLit(x) => x.inner(),
		}
	}
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub enum InfixOperator {
	Plus(Plus),
	Minus(Minus),
}
impl std::fmt::Debug for InfixOperator {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::Plus(x) => std::fmt::Debug::fmt(x, f),
			Self::Minus(x) => std::fmt::Debug::fmt(x, f),
		}
	}
}
impl AstNode for InfixOperator {}
impl AstElement for InfixOperator {
	fn can_cast(kind: SyntaxKind) -> bool { matches!(kind, |SyntaxKind::Plus| SyntaxKind::Minus) }

	fn cast(elem: SyntaxElement) -> Option<Self> {
		match elem.kind() {
			SyntaxKind::Plus => AstElement::cast(elem.clone()).map(Self::Plus),
			SyntaxKind::Minus => AstElement::cast(elem.clone()).map(Self::Minus),
			_ => None,
		}
	}

	fn span(&self) -> FileSpan {
		match self {
			Self::Plus(x) => x.span(),
			Self::Minus(x) => x.span(),
		}
	}

	fn inner(self) -> SyntaxElement {
		match self {
			Self::Plus(x) => x.inner(),
			Self::Minus(x) => x.inner(),
		}
	}
}
