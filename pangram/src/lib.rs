/// Determine whether a sentence is a pangram.
pub fn is_pangram(sentence: &str) -> bool {
    let mut set = [0u8; 26];
    for c in sentence.as_bytes() {
        let mut c = *c;
        if !c.is_ascii_alphabetic() {
            continue;
        }
        if c.is_ascii_uppercase() {
            c = c.to_ascii_lowercase();
        }
        set[(c - 'a' as u8) as usize] = 1;
    }
    for n in set.iter() {
        if *n != 1 {
            return false;
        }
    }
    true
}
