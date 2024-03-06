// Standard Uses
use std::{path::Path, process::Command};

// External Uses
use ungrammar::{Grammar, Rule};



pub fn pluralize(x: &str) -> String {
	let mut s = String::with_capacity(x.len() + 1);
	s.push_str(x);

	if s.ends_with('_') {
		s.pop();
	}

	s.push('s');
	s
}

pub fn format_rule<'a>(rule: &'a Rule, grammar: &'a Grammar) -> impl std::fmt::Display + 'a {
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

pub fn format(file: &Path) { Command::new("rustfmt").arg(file).status().unwrap(); }


