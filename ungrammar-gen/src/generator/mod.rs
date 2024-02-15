// Relative Modules
mod gen;
mod utils;
mod nodes;

// Standard Uses
use std::path::Path;

// Local Uses
use crate::generator::gen::Generator;

// External Uses
use eyre::Result;
use ungrammar::Grammar;
use ungrammar_extra::KindsMetaInfo;



pub fn from_path<M: KindsMetaInfo + 'static>(grammar_path: &Path, output_path: &Path) -> Result<()> {
    let grammar: Grammar = std::fs::read_to_string(grammar_path)?.parse()?;

    Generator::<M>::new(&grammar).generate(&output_path);
    Ok(())
}
