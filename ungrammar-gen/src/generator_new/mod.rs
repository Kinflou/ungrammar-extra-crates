// Relative Modules

// Standard Uses
use std::path::Path;

// Local Uses

// External Uses
use eyre::Result;
use ungrammar::Grammar;
use ungrammar_extra::KindsMetaInfo;

#[allow(unused)]
pub fn from_path<M: KindsMetaInfo>(grammar_path: &Path, output_path: &Path) -> Result<()> {
	let grammar: Grammar = std::fs::read_to_string(grammar_path)?.parse()?;

	Ok(())
}
