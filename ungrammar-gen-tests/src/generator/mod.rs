#[cfg(test)]
pub mod calculator;

// External Uses
use ungrammar_extra_derive::SyntaxKind;


#[allow(unused)]
#[derive(SyntaxKind)]
pub enum TokenKind {
	#[syntax(lit="string", desc="String")]
	StringLit,

	#[syntax(lit="+", desc="Plus")]
	PlusOp,
	
	#[syntax(lit="-", desc="Minus")]
	MinusOp,

	#[syntax(lit="int", desc="Integer")]
	IntLit,
	
	#[syntax(lit="float", desc="Float")]
	FloatLit,
}
