// Standard Uses
use std::{collections::HashMap, marker::PhantomData, path::Path};

// Crate Uses
use super::utils::{
	format::{format_rule, pluralize},
	io::{self, Uses}, lint::to_snake_case,
};

// External Uses
use quote::{format_ident, quote};
use ungrammar::{Grammar, Node, Rule};
use ungrammar_extra::KindsMetaInfo;



pub enum NodeType {
	Enum,
	Struct,
}

pub enum NodeData {
	Struct(Struct),
	Enum(Enum),
}

pub struct Struct {
	pub name: String,
	pub fields: Vec<Field>,
	pub type_cardinality: HashMap<String, Cardinality>,
}

impl Struct {
	fn get_cardinality(&mut self, ty: &String, rule: &Rule, grammar: &Grammar) -> usize {
		match self.type_cardinality.get_mut(ty) {
			Some(Cardinality::One(x)) => {
				*x += 1;
				*x
			},
			Some(Cardinality::Many) => panic!(
				"rule `{}` uses type `{}` which was already used before",
				format_rule(rule, grammar),
				ty
			),
			None => {
				self.type_cardinality.insert(ty.clone(), Cardinality::One(0));
				0
			},
		}
	}

	fn use_many_cardinality(&mut self, ty: &str, r: &Rule, grammar: &Grammar) {
		if self.type_cardinality.contains_key(ty) {
			panic!(
				"rule `{}` uses type `{}` which was already used before",
				format_rule(r, grammar),
				ty
			);
		}
	}
}

pub enum Field {
	Token {
		name: String,
		ty: String,
		cardinality: Cardinality,
	},
	Node {
		name: String,
		ty: String,
		cardinality: Cardinality,
	},
}

pub enum Cardinality {
	One(usize),
	Many,
}

pub struct Variant {
	pub name: String,
	pub node: Node,
}

pub struct Enum {
	pub name: String,
	pub node_variants: Vec<Variant>,
	pub token_variants: Vec<String>,
}

pub struct Generator<'a, M: KindsMetaInfo> {
	grammar: &'a Grammar,
	pub node_types: HashMap<Node, NodeType>,
	meta: PhantomData<M>,
}

impl<'a, M: KindsMetaInfo + 'static> Generator<'a, M> {
	pub fn new(grammar: &'a Grammar) -> Self {
		Self {
			grammar,
			node_types: HashMap::new(),
			meta: Default::default(),
		}
	}

	pub fn generate(mut self, out: &Path) {
		self.init_node_types();

		io::write_generated(out, "kind.rs", None, self.gen_kinds());
		io::write_generated(
			out,
			"token.rs",
			Some(Uses {
				mods: vec![],
				std: vec!["super::{*, nodes::*}".to_owned()],
				krate: vec![],
				external: vec!["diagnostics::FileSpan".to_owned()]
			}),
			self.gen_tokens(),
		);
		io::write_generated(
			out,
			"ast.rs",
			Some(Uses {
				mods: vec![],
				std: vec!["super::{*, token::*, nodes::*, blanket_impls}".to_owned()],
				krate: vec![],
				external: vec!["diagnostics::FileSpan".to_owned()] 
			}),
			super::ast::generate_ast(&mut self),
		);

		self.gen_ast_traits(out);
	}

	fn gen_kinds(&mut self) -> String {
		let token_kinds: Vec<_> = self
			.grammar
			.tokens()
			.map(|n| map_token::<M>(&self.grammar[n].name))
			.chain(["Whitespace", "Comment", "Error"].map(|s| (s.to_ascii_lowercase(), s.to_string())))
			.collect();

		let node_kinds: Vec<_> = self
			.grammar
			.iter()
			.filter_map(|n| match self.node_types[&n] {
				NodeType::Struct => Some(self.grammar[n].name.to_string()),
				NodeType::Enum => None,
			})
			.collect();

		let token_display: Vec<_> = token_kinds.iter().map(|(x, _)| x).collect();
		let token_kinds: Vec<_> = token_kinds.iter().map(|(_, x)| format_ident!("{}", x)).collect();
		let node_display: Vec<_> = node_kinds.iter().collect();
		let node_kinds: Vec<_> = node_kinds.iter().map(|x| format_ident!("{}", x)).collect();

		let def = quote! {
			#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, cstree::Syntax)]
			#[repr(u32)]
			pub enum SyntaxKind {
				/// Terminal tokens
				#(#token_kinds,)*

				/// Non-terminal nodes
				#(#node_kinds,)*

				#[doc(hidden)]
				Eof,
			}
		};

		let display = quote! {
			impl std::fmt::Display for SyntaxKind {
				fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
					match self {
						#(Self::#token_kinds => write!(f, #token_display),)*
						#(Self::#node_kinds => write!(f, #node_display),)*
						Self::Eof => write!(f, "<eof>"),
					}
				}
			}
		};

		let from = quote! {
			impl From<lex::token::TokenKind> for SyntaxKind {
				fn from(kind: lex::token::TokenKind) -> Self {
					match kind {
						#(lex::token::TokenKind::#token_kinds => Self::#token_kinds,)*
						lex::token::TokenKind::Eof => Self::Eof,
					}
				}
			}
		};

		def.to_string() + "\n\n" + &display.to_string() + "\n\n" + &from.to_string()
	}

	fn gen_tokens(&mut self) -> String {
		self.grammar
			.tokens()
			.map(|n| {
				{
				
					let ident = format_ident!("{}", map_token::<M>(&self.grammar[n].name).1);

					quote! {
						#[derive(Clone, PartialEq, Eq, Hash)]
						pub struct #ident(SyntaxToken);

						impl std::fmt::Debug for #ident {
							fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
								std::fmt::Debug::fmt(&self.0, f)
							}
						}

						impl AstToken for #ident {
							fn text(&self) -> Text { unsafe { std::mem::transmute(self.0.text_key().unwrap()) } }
						}

						impl AstElement for #ident {
							fn can_cast(kind: SyntaxKind) -> bool {
								kind == SyntaxKind::#ident
							}

							fn cast(elem: SyntaxElement) -> Option<Self> {
								let tok = elem.into_token()?;
								Self::can_cast(tok.kind()).then(|| Self(tok))
							}

							fn span(&self) -> FileSpan { blanket_impls::default_span(self.0) }

							fn inner(self) -> SyntaxElement { self.0.into() }
						}
					}
				}
				.to_string()
			})
			.intersperse("\n\n".to_string())
			.collect()
	}

	fn init_node_types(&mut self) {
		self.node_types = self
			.grammar
			.iter()
			.map(|x| {
				let node = &self.grammar[x];
				(
					x,
					match node.rule {
						Rule::Alt(_) => NodeType::Enum,
						_ => NodeType::Struct,
					},
				)
			})
			.collect();
	}

	pub fn gen_nodes(&mut self) -> Vec<NodeData> {
		const HANDWRITTEN: &[&str] = &["TokenTree"];

		self.grammar
			.iter()
			.filter(|x| !HANDWRITTEN.contains(&self.grammar[*x].name.as_str()))
			.map(|x| {
				let node = &self.grammar[x];
				match self.node_types[&x] {
					NodeType::Struct => {
						let mut s = Struct {
							name: node.name.clone(),
							fields: Vec::new(),
							type_cardinality: HashMap::new(),
						};
						self.lower_rule(&mut s, None, &node.rule);
						NodeData::Struct(s)
					},
					NodeType::Enum => {
						let e = self.lower_enum(node.name.clone(), &node.rule);
						NodeData::Enum(e)
					},
				}
			})
			.collect()
	}

	fn lower_rule(&mut self, out: &mut Struct, label: Option<&String>, rule: &Rule) {
		if self.lower_comma_list(out, label, rule) {
			return;
		}

		match rule {
			Rule::Labeled { label, rule } => self.lower_rule(out, Some(label), rule),
			Rule::Node(node) => {
				let ty = self.grammar[*node].name.clone();
				let index = out.get_cardinality(&ty, rule, self.grammar);

				out.fields.push(Field::Node {
					name: label.cloned().unwrap_or_else(|| to_snake_case(&ty)),
					ty,
					cardinality: Cardinality::One(index),
				});
			},
			Rule::Token(tok) => {
				let tok = &self.grammar[*tok].name;
				let ty = map_token::<M>(tok).1;
				let index = out.get_cardinality(&ty, rule, self.grammar);

				out.fields.push(Field::Token {
					name: label.cloned().unwrap_or_else(|| to_snake_case(&ty)),
					ty,
					cardinality: Cardinality::One(index),
				});
			},
			Rule::Seq(rules) => {
				for rule in rules {
					self.lower_rule(out, label, rule);
				}
			},
			Rule::Alt(_) => panic!(
				"rule `{}` is not allowed in this position",
				format_rule(rule, self.grammar)
			),
			Rule::Opt(rule) => self.lower_rule(out, label, rule),
			Rule::Rep(rule) => {
				if let Rule::Node(node) = &**rule {
					let ty = self.grammar[*node].name.clone();
					out.use_many_cardinality(&ty, rule, self.grammar);

					out.fields.push(Field::Node {
						name: label.cloned().unwrap_or_else(|| pluralize(&to_snake_case(&ty))),
						ty,
						cardinality: Cardinality::Many,
					});
				}
			},
		}
	}

	fn lower_enum(&mut self, name: String, rule: &Rule) -> Enum {
		let mut node_variants = Vec::new();
		let mut token_variants = Vec::new();

		let alt = match rule {
			Rule::Alt(alt) => alt,
			_ => panic!("expected an alt rule"),
		};

		for alt in alt {
			match alt {
				Rule::Node(node) => {
					let data = &self.grammar[*node];
					node_variants.push(Variant {
						name: data.name.clone(),
						node: *node,
					});
				},
				Rule::Token(tok) => {
					let tok = map_token::<M>(&self.grammar[*tok].name).1;
					token_variants.push(tok);
				},
				_ => panic!(
					"rule `{}` is not allowed in this position",
					format_rule(rule, self.grammar)
				),
			}
		}

		Enum {
			name,
			node_variants,
			token_variants,
		}
	}

	// (T (',' T)* ','?)
	// Stolen from rust-analyzer
	fn lower_comma_list(&mut self, out: &mut Struct, label: Option<&String>, r: &Rule) -> bool {
		let rule = match r {
			Rule::Seq(it) => it,
			_ => return false,
		};
		let (node, repeat, trailing_comma) = match rule.as_slice() {
			[Rule::Node(node), Rule::Rep(repeat), Rule::Opt(trailing_comma)] => (node, repeat, trailing_comma),
			_ => return false,
		};
		let repeat = match &**repeat {
			Rule::Seq(it) => it,
			_ => return false,
		};
		match repeat.as_slice() {
			[comma, Rule::Node(n)] if comma == &**trailing_comma && n == node => (),
			_ => return false,
		}
		let ty = self.grammar[*node].name.clone();
		let name = label.cloned().unwrap_or_else(|| pluralize(&to_snake_case(&ty)));
		out.use_many_cardinality(&ty, r, self.grammar);

		out.fields.push(Field::Node {
			name,
			ty,
			cardinality: Cardinality::Many,
		});

		true
	}
}

fn map_token<M: KindsMetaInfo>(token: &str) -> (String, String) {
	let Some((lit, desc)) = M::kinds().get_key_value(token) else {
		let mut message = format!("No token kind information found for literal '{token}'\n");
		message += "Available literals are:\n";

		for (lit, desc) in M::kinds() {
			message += &*format!("  - {lit} ({desc})\n");
		}

		panic!("{}", message)
	};

	(lit.to_string(), desc.to_string())
}

/*
fn map_token<M: KindsMetaInfo>(token: &TokenData) -> (String, String) {
	// TODO: Take the parent rule as parameter, if the parent is given the panic
	//       token information can tell who is the parent and maybe also
	//       at which line/position it is.
	//       Optionally also get the `token` parameter full information for more accuracy

	let Some((lit, desc)) = M::kinds().get_key_value(&*token.name) else {
		let mut message = format!("No token kind information found for literal '{}'\n", token.name);
		message += &*format!("Token is at line {}", token.);
		message += "Available literals are:\n";

		for (lit, desc) in M::kinds() {
			message += &*format!("  - {lit} ({desc})");
		}

		panic!("{}", message)
	};

	(lit.to_string(), desc.to_string())
}
*/