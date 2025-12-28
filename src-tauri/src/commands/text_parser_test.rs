#[cfg(test)]
mod tests {
    use crate::commands::text_parser::parse_task_text;

    #[test]
    fn test_parse_simple_list() {
        let input = "src/main.rs\nsrc/lib.rs";
        let result = parse_task_text(input).expect("Failed to parse");
        assert_eq!(result.len(), 2);
        assert_eq!(result[0].file, "src/main.rs");
        assert_eq!(result[1].file, "src/lib.rs");
    }

    #[test]
    fn test_parse_with_comments() {
        let input = "src/main.rs\tCheck this file\nsrc/lib.rs";
        let result = parse_task_text(input).expect("Failed to parse");
        assert_eq!(result.len(), 2);
        assert_eq!(result[0].file, "src/main.rs");
        assert_eq!(result[0].preset_comment, Some("Check this file".to_string()));
    }
}
