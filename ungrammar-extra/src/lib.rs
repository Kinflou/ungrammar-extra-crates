// Standard USes
use std::collections::HashMap;

// Local Uses

// External Uses


pub struct SyntaxKindMeta {
    pub name: String,
    pub literal: String,
    pub description: String
}

pub trait KindsMetaInfo {
    fn literals() -> &'static [&'static str];

    fn kinds() -> &'static HashMap<&'static str, &'static str>;
    fn descriptions() -> &'static HashMap<&'static str, &'static str>;

    /*
    fn kind(name: &str) -> Option<&str> {
        Self::kinds().iter().filter(|k| **k == name).next().copied()
    }
    */
}
