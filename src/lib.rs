pub mod parsers;

pub fn strip_formatting(str: &str) -> String {
    let mut result = String::new();
    let mut chars = str.chars();
    while let Some(c) = chars.next() {
        if c == '§' {
            chars.next().unwrap();
        } else {
            result.push(c);
        }
    }
    result
}
