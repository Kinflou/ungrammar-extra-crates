// Standard Uses
use std::path::Path;

// External Uses
use ungrammar_extra_derive::SyntaxKind;
use ungrammar_gen::generator;



#[test]
fn parse_calculator_ungrammar_and_generate_code() {
	#[allow(unused)]
	#[derive(SyntaxKind)]
	enum TokenKind {
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

	let grammar_path = Path::new("tests/_data_/calculator.ungram");
	let output_path = Path::new("tests/_temp_/generator/calculator/");

	if !output_path.exists() {
		std::fs::create_dir_all(output_path).unwrap();
	}

	generator::from_path::<SyntaxKind>(&grammar_path, &output_path).unwrap();
}
