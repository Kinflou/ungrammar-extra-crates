// Standard Uses
use std::path::Path;

// External Uses
use ungrammar_extra_derive::SyntaxKind;



#[test]
fn generate_with_macroed_token_kind_enum() {
	#[allow(unused)]
	#[derive(SyntaxKind)]
	enum TokenKind {
		#[syntax(lit = "string", desc = "String")]
		StringLit,
	}

	let grammar_path = Path::new("_data_/calculator.ungram");
	let output_path = Path::new("tests/generator_new/calculator/");

	if !output_path.exists() {
		std::fs::create_dir_all(output_path).unwrap();
	}

	ungrammar_gen::generator_new::from_path::<SyntaxKind>(grammar_path, output_path).unwrap();
}
