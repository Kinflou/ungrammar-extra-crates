// Relative Modules
mod ast;
mod gen;
mod nodes;
mod utils;

// Standard Uses
use std::path::Path;

// External Uses
use eyre::Result;
use ungrammar::Grammar;
use ungrammar_extra::KindsMetaInfo;

// Local Uses
use crate::generator::gen::Generator;

pub fn from_path<M: KindsMetaInfo + 'static>(grammar_path: &Path, output_path: &Path) -> Result<()> {
	let grammar: Grammar = std::fs::read_to_string(grammar_path)?.parse()?;

	Generator::<M>::new(&grammar).generate(&output_path);
	Ok(())
}
