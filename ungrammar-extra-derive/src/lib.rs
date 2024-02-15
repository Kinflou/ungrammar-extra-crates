// Relative Modules
mod attribute;
mod derive_enum;
mod into_kind_impl;
mod parse;

use std::str::FromStr;

// Standard Uses
use proc_macro::TokenStream;

// Crate Uses
use crate::derive_enum::DeriveEnum;

// External Uses
use virtue::prelude::*;

#[proc_macro_derive(SyntaxKind, attributes(syntax))]
pub fn syntax_kind_from_token_kind_derive(
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    syntax_kind_from_token_kind(input).unwrap_or_else(|error| error.into_token_stream())
}

fn syntax_kind_from_token_kind(input: TokenStream) -> Result<TokenStream> {
    let parse = Parse::new(input)?;
    let (mut generator, _, body) = parse.into_generator();

    let derive_enum;

    match body {
        Body::Enum(body) => {
            derive_enum = Some(DeriveEnum::new(body.variants).generate_encode_new(&mut generator)?);
        }
        Body::Struct(_) => {
            panic!("Only derive SyntaxKind an enum with Tokens for lexing (TokenKind)")
        }
    }

    // generator.export_to_file("ungrammar-extra-derive", "SyntaxKind");

    let mut enum_variants = vec![];
    for variant in &derive_enum.unwrap().variants {
        enum_variants.push(attribute::parse_tokens_info_attribute(
            &variant.attributes,
            variant.name.to_string(),
        )?);
    }
    
    let mut output = TokenStream::from_str(
        &*derive_enum::generate_kinds_meta_info(&enum_variants)?
    ).unwrap();
    output.extend(generator.finish()?);

    Ok(output)
}
