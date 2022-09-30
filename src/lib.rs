use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

pub fn calculate_hash<T: Hash>(pattern: &T) -> u64 {
    let mut default_hasher = DefaultHasher::new();
    pattern.hash(&mut default_hasher);

    default_hasher.finish()
}

pub fn weak_hash<T: AsRef<str>>(pattern: T) -> u64 {
    pattern
        .as_ref()
        .as_bytes()
        .iter()
        .map(|x| *x as u64)
        .sum::<u64>()
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

pub fn rolling_sum(prev_hash: u64, prev_window: &str, actual_window: &str) -> u64 {
    prev_hash
        - *prev_window.as_bytes().first().unwrap_or(&0) as u64
        + *actual_window.as_bytes().last().unwrap_or(&0) as u64
}

pub fn rolling_hash(text: &str, pattern: &str) -> Option<usize> {
    if pattern.len() > text.len() {
        return None
    }

    let patern_hash = weak_hash(pattern);
    let mut window_hash = weak_hash(&text[0..pattern.len()]);

    if patern_hash == window_hash {
        return Some(0)
    }

    for i in 1..=(text.len() - pattern.len()) {
        let prev_window = &text[(i-1)..(pattern.len() + i)];
        let actual_window = &text[i..(pattern.len() + i)];
        window_hash = rolling_sum(window_hash, prev_window, actual_window);

        if patern_hash == window_hash {
            return Some(i);
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

    #[test]
    fn test_weak_hash_beginning() {
        let pattern = "abcd";
        let result = weak_hash(pattern);
        assert_eq!(result, 394);
    }

    #[test]
    fn test_weak_hash_end() {
        let pattern = "wxyz";
        let result = weak_hash(pattern);
        assert_eq!(result, 482);
    }

    #[test]
    fn test_weak_hash() {
        let pattern = "abcd";
        let result = weak_hash(pattern);
        assert_eq!(result, 394);
    }

    #[test]
    fn test_rolling_sum() {
        let prev_hash = 394_u64;
        let prev_window = "abcd";
        let actual_window = "bcde";
        let result = rolling_sum(prev_hash, prev_window, actual_window);
        assert_eq!(result, 398);
    }

    #[test]
    fn test_rolling_hash() {
        let text = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaab";
        let pattern = "aaaaaaaaaab";
        let result = rolling_hash(text, pattern);
        assert_eq!(result, Some(44));
    }
}
