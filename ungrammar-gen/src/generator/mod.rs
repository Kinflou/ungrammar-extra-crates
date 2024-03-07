// Relative Modules
mod ast;
mod gen;
mod nodes;
mod blanket_impls;

mod utils;

// Standard Uses
use std::path::Path;

// Crate Uses
use crate::generator::gen::Generator;

// External Uses
use eyre::{Context, Result};
use ungrammar::Grammar;
use ungrammar_extra::KindsMetaInfo;



pub fn from_path<M: KindsMetaInfo + 'static, K>(
	grammar_path: &Path, output_path: &Path
) -> Result<()> {
	let file = std::fs::read_to_string(grammar_path)
		.with_context(|| format!("Couldn't read file at path '{}'", grammar_path.display()))?;
	let grammar: Grammar = file.parse()?;

	if !output_path.exists() {
		println!("Creating generated code path at {}", output_path.display());
		std::fs::create_dir_all(output_path)?;
	}

	Generator::<M>::new(&grammar, core::any::type_name::<K>().to_owned())
		.generate(&output_path);

	Ok(())
}
