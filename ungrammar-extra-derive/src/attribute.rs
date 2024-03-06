// Standard Uses

// Crate Uses

// External Uses
use virtue::parse::Attribute;
use virtue::prelude::*;
use virtue::utils::{ParsedAttribute, parse_tagged_attribute};

pub struct TokenInfo {
    pub literal: String,
    pub description: Option<String>,
    pub variant_name: String,
}

pub fn parse_tokens_info_attribute(
    attrs: &[Attribute],
    variant_name: String,
) -> Result<Option<TokenInfo>> {
    let (mut lit, mut desc) = (None, None);

    for attr in attrs {
        let Some(attributes) = parse_tagged_attribute(&attr.tokens, "syntax")? else {
            continue;
        };

        for attr in attributes {
            match attr {
                ParsedAttribute::Property(key, val) => match &*key.to_string() {
                    "lit" => lit = Some(val.to_string()),
                    "desc" => desc = Some(val.to_string()),
                    k => panic!("Unexpected key: {:?}", k),
                },
                x => panic!("Unexpected attribute: {:?}", x),
            }
        }
    }

    if lit.is_none() {
        if !variant_name.is_ascii() {
            panic!(
                "Literal contains numbers, it can only be alphabetic(A to Z letters) at variant'{variant_name}'."
            )
        }
        
        let mut s = String::with_capacity(variant_name.len() + 2);
        s.push_str(&variant_name[0..1].to_ascii_uppercase());
        s.push_str(&variant_name[1..]);
        s.push_str("Kw");

        //lit = Some(backtick(&*s));
        lit = Some(s);


        /*
        panic!(
            "Literal was not specified at variant '{variant_name}'.\n\
            Try doing for example:\n\
            #[syntax(lit=\"...\", ...)]\n{variant_name}"
        )
        */
    }

    Ok(Some(TokenInfo {
        literal: lit.unwrap(),
        description: desc,
        variant_name,
    }))
}

/*
fn backtick(x: &str) -> String {
    let mut s = String::with_capacity(x.len() + 2);
    s.push('`');
    s.push_str(x);
    s.push('`');
    s
}
*/
