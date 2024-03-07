// Standard Uses

// Crate Uses
use crate::attribute::{self, TokenInfo};

// External Uses
use virtue::prelude::*;

pub(crate) struct DeriveEnum {
    pub variants: Vec<EnumVariant>,
}

impl DeriveEnum {
    pub fn new(variants: Vec<EnumVariant>) -> Self {
        Self { variants }
    }
    pub fn generate_encode_new(self, generator: &mut Generator) -> Result<Self> {
        let mut attrs = vec![];

        for variant in &self.variants {
            attrs.push(attribute::parse_tokens_info_attribute(
                &variant.attributes,
                variant.name.to_string(),
            )?);
        }

        // TODO: This might not be necessary, or may be optional, decide later if to make this
        //       optional for the user to opt-out
        self.generate_syntax_kind_enum(generator)?;

        self.generate_kinds_meta_info_trait_impl(generator, &attrs)?;

        Ok(self)
    }

    fn generate_syntax_kind_enum(&self, generator: &mut Generator) -> Result<()> {
        let mut kind_enum = generator.generate_enum("SyntaxKind");
        let kind_enum = kind_enum.make_pub();

        for variant in &self.variants {
            kind_enum.add_value(variant.name.to_string()).make_zst();
        }

        Ok(())
    }

    /// Generate Kinds meta information implementation, this information is used by
    /// the ungrammar codegen later at build stage
    fn generate_kinds_meta_info_trait_impl(
        &self,
        generator: &mut Generator,
        attrs: &[Option<TokenInfo>],
    ) -> Result<()> {
        let prefix = "TOKENKIND_";

        /*
        generator
            .generate_fn(format!("{prefix}LITERALS"))
            .with_return_type(
                "once_cell::sync::Lazy<std::collections::HashMap<&'static str, &'static str>>",
            )
            .body(|b| {
                let mut body =
                    "once_cell::sync::Lazy::new(|| std::collections::HashMap::from([".to_string();

                for attr in attrs {
                    match attr {
                        Some(attr) => {
                            body += &*format!("\t({}, \"{}\"), \n", attr.literal, attr.variant_name)
                        }
                        None => {
                            todo!("rust bad err#1")
                        }
                    }
                }

                body += "]))";

                b.push_parsed(body)?;
                Ok(())
            })?;

        meta_impl
            .generate_fn(format!("{prefix}DESCRIPTIONS"))
            .with_return_type(
                "once_cell::sync::Lazy<std::collections::HashMap<&'static str, &'static str>>",
            )
            .body(|b| {
                let mut body =
                    "once_cell::sync::Lazy::new(|| std::collections::HashMap::from([".to_string();

                for attr in attrs {
                    match attr {
                        Some(attr) => {
                            let desc = attr.description.as_ref().unwrap_or(&attr.literal);
                            body += &*format!("\t({}, {desc}), \n", attr.literal);
                        }
                        None => {
                            todo!("rust bad err#2")
                        }
                    }
                }

                body += "]))";

                b.push_parsed(body)?;
                Ok(())
            })?;
        
        */
        
        let mut meta_impl =
        generator.impl_trait_for_other_type("ungrammar_extra::KindsMetaInfo", "SyntaxKind");

        meta_impl
            .generate_fn("literals")
            .with_return_type("&'static [&'static str]")
            .body(|b| {
                let mut body = "&[".to_string();

                for attr in attrs {
                    match attr {
                        Some(attr) => {
                            body += &*format!("\t{},\n", attr.literal);
                        }
                        None => {
                            todo!("rust bad err#3")
                        }
                    }
                }

                body += "]";

                b.push_parsed(body)?;
                Ok(())
            })?;

        meta_impl
            .generate_fn("kinds")
            .with_return_type("&'static std::collections::HashMap<&'static str, &'static str>")
            .body(|b| {
                b.push_parsed(format!("&*{prefix}KINDS"))?;
                Ok(())
            })?;

        meta_impl
            .generate_fn("descriptions")
            .with_return_type("&'static std::collections::HashMap<&'static str, &'static str>")
            .body(|b| {
                b.push_parsed(format!("&*{prefix}DESCRIPTIONS"))?;
                Ok(())
            })?;

        Ok(())
    }
}


pub fn generate_kinds_meta_info(attrs: &[Option<TokenInfo>]) -> Result<String> {
    let prefix = "TOKENKIND_";
    
    let mut kinds = format!(
        "static {}KINDS: once_cell::sync::Lazy<std::collections::HashMap<&'static str, &'static str>> = once_cell::sync::Lazy::new(|| std::collections::HashMap::from([",
        prefix
    );

    let mut descriptions = format!(
        "static {}DESCRIPTIONS: once_cell::sync::Lazy<std::collections::HashMap<&'static str, &'static str>> = once_cell::sync::Lazy::new(|| std::collections::HashMap::from([",
        prefix
    );        

    for attr in attrs {
        match attr {
            Some(attr) => {
                kinds += &*format!(
                    "\t({}, \"{}\"), \n", attr.literal, attr.variant_name
                );

                let desc = attr.description.clone().unwrap_or(
                    format!("\"{}\"", &attr.variant_name)
                );
                descriptions += &*format!("\t({}, {desc}), \n", attr.literal);
            }
            None => {
                todo!("rust bad err#1")
            }
        }
    }

    kinds += "]));";
    descriptions += "]));";

    Ok(format!("{}\n{}\n\n", kinds, descriptions))
}

