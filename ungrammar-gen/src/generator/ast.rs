// Standard Uses

// Crate Uses
use super::gen::{Enum, Generator, NodeData, Struct};
use crate::generator::gen::{Cardinality, Field, NodeType};

// External Uses
use quote::{format_ident, quote};
use ungrammar_extra::KindsMetaInfo;



pub fn generate_ast<'a, M: KindsMetaInfo + 'static>(generator: &mut Generator<'a, M>) -> String {
	let nodes = generator.gen_nodes();

	nodes
		.into_iter()
		.map(|node| {
			match node {
				NodeData::Struct(s) => generate_struct(generator, s),
				NodeData::Enum(e) => generate_enum(generator, e),
			}
			.to_string()
		})
		.intersperse("\n\n".to_string())
		.collect()
}

fn generate_struct<'a, M: KindsMetaInfo>(_: &mut Generator<'a, M>, s: Struct) -> proc_macro2::TokenStream {
	let name = format_ident!("{}", s.name);
	let fields = s.fields.into_iter().map(|f| match f {
		Field::Node { name, ty, cardinality } => {
			let name = format_ident!("{}", name);
			let ty = format_ident!("{}", ty);
			match cardinality {
				Cardinality::Many => quote! {
					pub fn #name(&self) -> impl Iterator<Item = #ty> + '_ {
						children(&self.0)
					}
				},
				Cardinality::One(n) => quote! {
					pub fn #name(&self) -> Option<#ty> {
						children(&self.0).nth(#n)
					}
				},
			}
		},
		Field::Token { name, ty, cardinality } => {
			let name = format_ident!("{}", name);
			let ty = format_ident!("{}", ty);
			match cardinality {
				Cardinality::Many => {
					quote! {
						pub fn #name(&self) -> impl Iterator<Item = #ty> + '_ {
							children(&self.0)
						}
					}
				},
				Cardinality::One(n) => {
					quote! {
						pub fn #name(&self) -> Option<#ty> {
							children(&self.0).nth(#n)
						}
					}
				},
			}
		},
	});
	
	quote! {
		#[derive(Clone, PartialEq, Eq, Hash)]
		pub struct #name(SyntaxNode);

		impl #name {
			#(#fields)*
		}
		
		impl AstNode for #name {}
		impl AstElement for #name {
			fn can_cast(kind: SyntaxKind) -> bool {
				kind == SyntaxKind::#name
			}
			fn cast(elem: SyntaxElement) -> Option<Self> {
				let node = elem.into_node()?;
				Self::can_cast(node.kind()).then(|| Self(node))
			}
			fn span(&self) -> FileSpan {
				let range = self.0.text_range();
				FileSpan {
					start: range.start().into(),
					end: range.end().into(),
					relative: (),
				}
			}
			fn inner(self) -> SyntaxElement { self.0.into() }
		}
		
		impl std::fmt::Debug for #name {
			fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { std::fmt::Debug::fmt(&self.0, f) }
		}
		
	}
}

fn generate_enum<'a, M: KindsMetaInfo>(generator: &mut Generator<'a, M>, e: Enum) -> proc_macro2::TokenStream {
	let name = format_ident!("{}", e.name);
	let token_variants: Vec<_> = e.token_variants.iter().map(|x| format_ident!("{}", x)).collect();

	let node_variants: Vec<_> = e.node_variants.iter().map(|x| format_ident!("{}", x.name)).collect();

	let mut struct_variants = Vec::new();
	let mut enum_variants = Vec::new();

	for x in e.node_variants {
		match generator.node_types[&x.node] {
			NodeType::Struct => struct_variants.push(format_ident!("{}", x.name)),
			NodeType::Enum => enum_variants.push(format_ident!("{}", x.name)),
		}
	}

	quote! {
		#[derive(Clone, PartialEq, Eq, Hash)]
		pub enum #name {
			#(#token_variants(#token_variants),)*
			#(#node_variants(#node_variants),)*
		}

		impl std::fmt::Debug for #name {
			fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
				match self {
					#(Self::#token_variants(x) => std::fmt::Debug::fmt(x, f),)*
					#(Self::#node_variants(x) => std::fmt::Debug::fmt(x, f),)*
				}
			}
		}

		impl AstNode for #name {}

		impl AstElement for #name {
			fn can_cast(kind: SyntaxKind) -> bool {
				matches!(kind, #(| SyntaxKind::#token_variants)* #(| SyntaxKind::#struct_variants)*)
				#(|| #enum_variants::can_cast(kind))*
			}

			fn cast(elem: SyntaxElement) -> Option<Self> {
				match elem.kind() {
					#(SyntaxKind::#struct_variants => AstElement::cast(elem.clone()).map(Self::#struct_variants),)*
					#(SyntaxKind::#token_variants => AstElement::cast(elem.clone()).map(Self::#token_variants),)*
					_ => None,
				} #(.or_else(|| AstElement::cast(elem.clone()).map(Self::#enum_variants)))*
			}

			fn span(&self) -> FileSpan {
				match self {
					#(Self::#token_variants(x) => x.span(),)*
					#(Self::#node_variants(x) => x.span(),)*
				}
			}

			fn inner(self) -> SyntaxElement {
				match self {
					#(Self::#token_variants(x) => x.inner(),)*
					#(Self::#node_variants(x) => x.inner(),)*
				}
			}
		}
	}
}
