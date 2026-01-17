pub fn apply_all(s: &str) -> String {
    let s = rule_lowercase(s);
    rule_long_vowels(&s)
}

fn rule_lowercase(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for ch in s.chars() {
        for lc in ch.to_lowercase() {
            out.push(lc);
        }
    }
    out
}

fn rule_long_vowels(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    let mut s_peek = s.chars().peekable();

    while let Some(c) = s_peek.next() {
        let macron = match (c, s_peek.peek().copied()) {
            ('i', Some('i')) => Some('ī'),
            ('e', Some('e')) => Some('ē'),
            ('o', Some('o')) => Some('ō'),
            ('u', Some('u')) => Some('ū'),
            ('a', Some('a')) => Some('ā'),
            _ => None,
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
