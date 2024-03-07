// Standard Uses
use std::path::{Path, PathBuf};

// External Uses
use eyre::Context;



pub fn write_generated(out: &Path, file: &str, uses: Option<Uses>, content: String) {
	let mut text = indoc::indoc! {r#"
    	// This file is generated, do not edit
		#![allow(clippy::all)]
		
		
	"#}.to_string();

	if let Some(uses) = uses {
		if uses.mods.len() > 0 {
			text += "// Relative Modules\n";
			for r#mod in uses.mods { text += &*format!("pub mod {};\n", r#mod) }
			text += "\n";
		}

		if uses.std.len() > 0 {
			//text += "// Standard Uses\n";
			for std in uses.std { text += &*format!("use {std};\n"); }
			text += "\n";
		}

		if uses.krate.len() > 0 {
			//text += "// Crate Uses\n";
			for krate in uses.krate { text += &*format!("use {krate};\n"); }
			text += "\n";
		}

		if uses.external.len() > 0 {
			//text += "// External Uses\n";
			for external in uses.external { text += &*format!("use {external};\n"); }
			text += "\n";			
		}
	}
	
	text += &*format!("\n\n{content}\n");

	write(out.join(file), text);
}

fn write(file: PathBuf, contents: String) {
	if file.file_name().is_none() {
		panic!("Given path has to be a file path.\nPath: {}", file.display());
	}

	std::fs::write(&file, contents)
		.with_context(|| format!("Could not write into file at: \n{}", file.display()))
		.unwrap();

	super::format::format(&file);
}


pub struct Uses {
	pub mods: Vec<String>,
	pub std: Vec<String>,
	pub krate: Vec<String>,
	pub external: Vec<String>
}
