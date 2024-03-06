
pub fn to_snake_case(x: &str) -> String {
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
