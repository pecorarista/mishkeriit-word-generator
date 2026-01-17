pub fn apply_all(input: &str) -> String {
    let rules: &[fn(String) -> String] = &[rule_lowercase, rule_glottal_stop, rule_long_vowels];
    rules.iter().fold(input.to_string(), |acc, rule| rule(acc))
}

fn rule_glottal_stop(s: String) -> String {
    s.replace('q', "'")
}

fn rule_lowercase(s: String) -> String {
    s.chars().flat_map(|ch| ch.to_lowercase()).collect()
}

fn rule_long_vowels(s: String) -> String {
    let mut out = String::with_capacity(s.len());
    let mut s_peek = s.chars().peekable();

    while let Some(c) = s_peek.next() {
        let macron = match (c, s_peek.peek().copied()) {
            ('i', Some('i')) => Some('ī'),
            ('e', Some('e')) => Some('ē'),
            ('o', Some('o')) => Some('ō'),
            ('u', Some('u')) => Some('ū'),
            ('a', Some('a')) => Some('ā'),
            _ => None
        };

        if let Some(m) = macron {
            out.push(m);
            s_peek.next();
        } else {
            out.push(c);
        }
    }
    out
}
