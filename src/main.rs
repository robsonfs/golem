pub fn brute_force(text: &str, pattern: &str) -> Option<usize> {
    println!("Pattern len: {}", pattern.as_bytes().len());
    println!("Text len: {}", text.as_bytes().len());
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

fn main() {
    let text = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaab";
    let pattern = "aaaaaaaaaab";

    println!("Result: {:?}", brute_force(text, pattern));
}
