// Standard Uses
use std::path::Path;


fn main() {
    //std::env::set_var("REBUILD", format!("{:?}", std::time::Instant::now()));
    //println!("cargo:rerun-if-env-changed=REBUILD");
    
    println!("Cleaning tests directory");
    std::fs::remove_dir_all(Path::new("tests")).ok();

    println!("Preparing tests directory.");
    std::fs::create_dir_all(Path::new("tests")).unwrap();
    std::fs::write("tests/mod.rs", "").unwrap();
}

