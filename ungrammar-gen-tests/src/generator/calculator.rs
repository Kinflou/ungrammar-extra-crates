// Standard Uses
use std::path::Path;

// Crate Uses
use super::GENERATED_CODE_PATH;

// External Uses
use ungrammar_gen::generator;



#[test]
fn parse_calculator_ungrammar_and_generate_code() {
	super::setup_generation();
	
	let grammar_path = Path::new("_data_/calculator.ungram");
	let output_path = GENERATED_CODE_PATH.join("generator/calculator/");

	if !output_path.exists() {
		std::fs::create_dir_all(&output_path).unwrap();
	}

	generator::from_path::<super::SyntaxKind, super::TokenKind>(&grammar_path, &output_path).unwrap();
}
