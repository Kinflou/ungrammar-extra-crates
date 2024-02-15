// This file is generated, do not edit

#![allow(clippy::all)]

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, cstree :: Syntax)]
#[repr(u32)]
pub enum SyntaxKind {
	/// Terminal tokens
	Plus,
	Minus,
	IntLit,
	FloatLit,
	Whitespace,
	Comment,
	Error,
	/// Non-terminal nodes
	Expr,
	#[doc(hidden)]
	Eof,
}

impl std::fmt::Display for SyntaxKind {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::Plus => write!(f, "`+`"),
			Self::Minus => write!(f, "`-`"),
			Self::IntLit => write!(f, "integer"),
			Self::FloatLit => write!(f, "floating-point number"),
			Self::Whitespace => write!(f, "whitespace"),
			Self::Comment => write!(f, "comment"),
			Self::Error => write!(f, "error"),
			Self::Expr => write!(f, "Expr"),
			Self::Eof => write!(f, "<eof>"),
		}
	}
}

impl From<lex::token::TokenKind> for SyntaxKind {
	fn from(kind: lex::token::TokenKind) -> Self {
		match kind {
			lex::token::TokenKind::Plus => Self::Plus,
			lex::token::TokenKind::Minus => Self::Minus,
			lex::token::TokenKind::IntLit => Self::IntLit,
			lex::token::TokenKind::FloatLit => Self::FloatLit,
			lex::token::TokenKind::Whitespace => Self::Whitespace,
			lex::token::TokenKind::Comment => Self::Comment,
			lex::token::TokenKind::Error => Self::Error,
			lex::token::TokenKind::Eof => Self::Eof,
		}
	}
}
