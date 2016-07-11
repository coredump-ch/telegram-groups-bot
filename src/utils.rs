//! Utility functions.

pub fn escape_markdown(text: &str) -> String {
    text.replace(r"\", "")
        .replace("*", r"\*")
        .replace("_", r"\_")
        .replace("`", r"\`")
        .replace("[", r"\[")
}


#[cfg(test)]
mod tests {
    use super::escape_markdown;

    #[test]
    fn test_strip_escaped() {
        assert_eq!(escape_markdown(r"hello\there"), r"hellothere");
    }

    #[test]
    fn test_escape_bold() {
        assert_eq!(escape_markdown(r"hello*there"), r"hello\*there");
    }

    #[test]
    fn test_escape_italic() {
        assert_eq!(escape_markdown(r"hello_there"), r"hello\_there");
    }

    #[test]
    fn test_escape_backtick() {
        assert_eq!(escape_markdown(r"hello`there"), r"hello\`there");
        assert_eq!(escape_markdown(r"```hello```"), r"\`\`\`hello\`\`\`");
    }

    #[test]
    fn test_escape_url() {
        assert_eq!(escape_markdown(r"[hello](http://example.com)"),
                                   r"\[hello](http://example.com)");
    }

    #[test]
    fn test_escape_all() {
        let input = r"hi\*the*re_ [url](dest) `code``";
        let out = escape_markdown(input);
        assert_eq!(out, r"hi\*the\*re\_ \[url](dest) \`code\`\`");
    }
}
