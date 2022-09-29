use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

pub fn calculate_hash<T: Hash>(pattern: &T) -> u64 {
    let mut default_hasher = DefaultHasher::new();
    pattern.hash(&mut default_hasher);

    default_hasher.finish()
}

pub fn brute_force(text: &str, pattern: &str) -> Option<usize> {
    if pattern.len() > text.len() {
        return None
    }

    let mut p: usize = 0;

    while p <= text.len() - pattern.len() {
        let mut found = false;
        for i in 0..pattern.len() {
            if pattern.as_bytes()[i] != text.as_bytes()[p + i] {
                continue;
            } else {
                if i == pattern.len() - 1 {
                    found = true;
                }
            }
        }
        if found {
            return Some(p)
        }
        p += 1;
    }
    return None
}

pub fn hash_approach(text: &str, pattern: &str) -> Option<usize> {
    if pattern.len() > text.len() {
        return None
    }

    let pattern_hash = calculate_hash(&pattern.as_bytes());

    for p in 0..=(text.len() - pattern.len()) {
        let end = pattern.len() + p;
        let partial_text_hash = calculate_hash(&text[p..end].as_bytes());
        if partial_text_hash == pattern_hash {
            return Some(p)
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_brute_force() {
        let text = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaab";
        let pattern = "aaaaaaaaaab";
        let result = brute_force(text, pattern);
        assert_eq!(result, Some(44));
    }

    #[test]
    fn test_hash_approach() {
        let text = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaab";
        let pattern = "aaaaaaaaaab";
        let result = hash_approach(text, pattern);
        assert_eq!(result, Some(44));
    }
}
