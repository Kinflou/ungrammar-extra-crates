// Standard Uses
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
    process::Command,
    marker::PhantomData,
};

// External Uses
use eyre::Context;
use quote::{format_ident, quote};
use ungrammar::{Grammar, Node, Rule};
use ungrammar_extra::KindsMetaInfo;



enum NodeType {
    Enum,
    Struct,
}

enum NodeData {
    Struct(Struct),
    Enum(Enum),
}

struct Struct {
    name: String,
    fields: Vec<Field>,
    type_cardinality: HashMap<String, Cardinality>,
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

enum Field {
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

enum Cardinality {
    One(usize),
    Many,
}

struct Variant {
    name: String,
    node: Node,
}

struct Enum {
    name: String,
    node_variants: Vec<Variant>,
    token_variants: Vec<String>,
}

pub struct Generator<'a, M: KindsMetaInfo> {
    grammar: &'a Grammar,
    node_types: HashMap<Node, NodeType>,
    meta: PhantomData<M>
}

impl<'a, M: KindsMetaInfo + 'static> Generator<'a, M> {
    pub fn new(grammar: &'a Grammar) -> Self {
        Self {
            grammar,
            node_types: HashMap::new(),
            meta: Default::default()
        }
    }

    pub(crate) fn write(out: &Path, file: &str, uses: Option<&str>, content: String) {
        let mut text = r#"
		// This file is generated, do not edit

		#![allow(clippy::all)]


		"#.to_string();

        if let Some(uses) = uses {
            text.push_str(uses);
            text.push_str("\n\n");
        }
        text.push_str(&content);
        write(out.join(file), text);
    }

    pub fn generate(mut self, out: &Path) {
        self.init_node_types();

        Self::write(out, "kind.rs", None, self.gen_kinds());
        Self::write(
            out,
            "token.rs",
            Some(
                "use super::{*, nodes::*};\n\
                use diagnostics::FileSpan;\n"
            ),
            self.gen_tokens(),
        );
        Self::write(
            out,
            "ast.rs",
            Some(
                "use super::{*, token::*, nodes::*, blanket_impls};\n\
                use diagnostics::FileSpan;"
            ),
            self.gen_ast(),
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

    fn gen_ast(&mut self) -> String {
        let nodes = self.gen_nodes();
        nodes
            .into_iter()
            .map(|node| {
                match node {
                    NodeData::Struct(s) => {
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

							impl std::fmt::Debug for #name {
								fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { std::fmt::Debug::fmt(&self.0, f) }
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

							impl #name {
								#(#fields)*
							}
						}
                    },
                    NodeData::Enum(e) => {
                        let name = format_ident!("{}", e.name);
                        let token_variants: Vec<_> = e.token_variants.iter().map(|x| format_ident!("{}", x)).collect();

                        let node_variants: Vec<_> =
                            e.node_variants.iter().map(|x| format_ident!("{}", x.name)).collect();

                        let mut struct_variants = Vec::new();
                        let mut enum_variants = Vec::new();

                        for x in e.node_variants {
                            match self.node_types[&x.node] {
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
                    },
                }
                    .to_string()
            })
            .intersperse("\n\n".to_string())
            .collect()
    }

    fn gen_nodes(&mut self) -> Vec<NodeData> {
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
    // TODO: Take the parent rule as parameter, if the parent is given the panic
    //       token information can tell who is the parent and maybe also
    //       at which line/position it is.
    //       Optionally also get the `token` parameter full information for more accuracy
    
    let Some((lit, desc)) = M::kinds().get_key_value(token) else {
        panic!("No token kind information found for literal '{token}'")
    };

    (lit.to_string(), desc.to_string())
}

fn to_snake_case(x: &str) -> String {
    const RUST_KEYWORDS: &[&str] = &[
        "abstract", "alignof", "as", "become", "box", "break", "const", "continue", "crate", "do", "else", "enum",
        "extern", "false", "final", "fn", "for", "if", "impl", "in", "let", "loop", "macro", "match", "mod", "move",
        "mut", "offsetof", "override", "priv", "proc", "pub", "pure", "ref", "return", "Self", "self", "sizeof",
        "static", "struct", "super", "trait", "true", "type", "typeof", "unsafe", "unsized", "use", "virtual", "where",
        "while", "yield",
    ];

    let mut s = String::with_capacity(x.len());
    let mut last = '_';
    for c in x.chars() {
        if c.is_ascii_uppercase() {
            if last != '_' {
                s.push('_');
            }
            s.push(c.to_ascii_lowercase());
        } else {
            s.push(c);
        }
        last = c;
    }

    if RUST_KEYWORDS.contains(&s.as_str()) {
        s.push('_');
    }

    s
}

fn pluralize(x: &str) -> String {
    let mut s = String::with_capacity(x.len() + 1);
    s.push_str(x);
    if s.ends_with('_') {
        s.pop();
    }
    s.push('s');
    s
}

fn format_rule<'a>(rule: &'a Rule, grammar: &'a Grammar) -> impl std::fmt::Display + 'a {
    struct Fmt<'a> {
        rule: &'a Rule,
        grammar: &'a Grammar,
    }

    impl std::fmt::Display for Fmt<'_> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self.rule {
                Rule::Labeled { label, rule } => write!(f, "{}:{}", label, format_rule(rule, self.grammar)),
                Rule::Node(node) => write!(f, "{}", self.grammar[*node].name),
                Rule::Token(tok) => write!(f, "'{}'", self.grammar[*tok].name),
                Rule::Seq(seq) => {
                    write!(f, "(")?;
                    for rule in seq {
                        write!(f, "{} ", format_rule(rule, self.grammar))?;
                    }
                    write!(f, ")")
                },
                Rule::Alt(opts) => {
                    write!(f, "(")?;
                    for rule in opts {
                        write!(f, "{} | ", format_rule(rule, self.grammar))?;
                    }
                    write!(f, ")")
                },
                Rule::Opt(v) => {
                    write!(f, "{}?", format_rule(v, self.grammar))
                },
                Rule::Rep(v) => {
                    write!(f, "{}*", format_rule(v, self.grammar))
                },
            }
        }
    }

    Fmt { rule, grammar }
}

fn write(file: PathBuf, contents: String) {
    std::fs::write(&file, contents)
        .with_context(|| format!("Could not write into file: {}", file.display()))
        .unwrap();
    format(&file);
}

fn format(file: &Path) { Command::new("rustfmt").arg(file).status().unwrap(); }