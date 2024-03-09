#[cfg(test)]
pub mod calculator;

// Standard Uses
use std::{io::Write, path::{Path, PathBuf}};

// External Uses
use once_cell::sync::Lazy;
use ungrammar_extra_derive::SyntaxKind;


pub static GENERATED_PATH: Lazy<PathBuf> = Lazy::new(|| Path::new("generated/").into());
pub static GENERATED_CODE_PATH: Lazy<PathBuf> = Lazy::new(|| Path::new("generated/src/").into());



pub fn setup_generation() {
	clean_generated_dir();
	generate_crate();

    //if !GENERATED_CODE_PATH.join("lib.rs").exists()
	{
		std::fs::OpenOptions::new().create(true).append(true).open(GENERATED_CODE_PATH.join("lib.rs"))
		.unwrap()
		.write_all(b"mod generator;\n").unwrap();
	}

    std::fs::create_dir_all(GENERATED_CODE_PATH.join("generator/")).ok();
    std::fs::write(
        GENERATED_CODE_PATH.join("generator/mod.rs"), "mod calculator;"
    ).unwrap();
}

fn generate_crate() {
	//std::fs::write(GENERATED_CODE_PATH.join("lib.rs"),"").unwrap();

	std::fs::write(
		GENERATED_PATH.join("Cargo.toml"),
		indoc::indoc!{r#"
		[package]
		name = "generated"
		version = "0.1.0"
		edition = "2021"

		[dependencies]
		ungrammar-gen-tests = { path="../" }
		ungrammar-extra = { path="../../ungrammar-extra" }
		cstree = { version = "0.12", features = ["derive", "multi_threaded_interning"] }

		[dependencies.text]
		git = "https://github.com/SparkyPotato/yam"
		rev = "ff948ab"

		[dependencies.diagnostics]
		git = "https://github.com/SparkyPotato/yam"
		rev = "ff948ab"

		[workspace]
		"#}
	).unwrap()
}

fn clean_generated_dir() {
	println!("Cleaning tests directory");
    std::fs::remove_dir_all(&*GENERATED_PATH).ok();

    println!("Preparing tests directory.");
    std::fs::create_dir_all(&*GENERATED_CODE_PATH).unwrap();
    std::fs::write(GENERATED_CODE_PATH.join("lib.rs"), "").unwrap();
}


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
