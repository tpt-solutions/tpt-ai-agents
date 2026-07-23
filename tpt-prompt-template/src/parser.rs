use crate::Error;

/// Template parser that extracts variables from template strings.
pub struct TemplateParser {
    variables: alloc::vec::Vec<alloc::string::String>,
}

impl TemplateParser {
    pub fn new() -> Self {
        Self {
            variables: alloc::vec::Vec::new(),
        }
    }

    pub fn parse(&mut self, template: &str) -> core::result::Result<(), Error> {
        self.variables.clear();
        let mut chars = template.char_indices().peekable();

        while let Some((i, c)) = chars.next() {
            if c == '{' && chars.peek() == Some(&(i + 1, '{')) {
                chars.next();
                let start = i + 2;
                let mut end = start;
                while let Some(&(pos, ch)) = chars.peek() {
                    if ch == '}' {
                        chars.next(); // consume first '}'
                        if chars.peek() == Some(&(pos + 1, '}')) {
                            chars.next(); // consume second '}'
                            let var_name = &template[start..end];
                            if var_name.is_empty() {
                                return Err(Error::UnclosedVariable { position: start });
                            }
                            if !var_name.chars().all(|c| c.is_alphanumeric() || c == '_') {
                                return Err(Error::InvalidVariableName(
                                    alloc::string::String::from(var_name),
                                ));
                            }
                            self.variables.push(alloc::string::String::from(var_name));
                            break;
                        }
                    }
                    end = pos + 1;
                    chars.next();
                }
            }
        }

        Ok(())
    }

    pub fn variables(&self) -> &[alloc::string::String] {
        &self.variables
    }
}

impl Default for TemplateParser {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_variables() {
        let mut parser = TemplateParser::new();
        parser
            .parse("Hello {{name}}, your topic is {{topic}}")
            .unwrap();
        assert_eq!(parser.variables(), &["name", "topic"]);
    }

    #[test]
    fn test_empty_template() {
        let mut parser = TemplateParser::new();
        parser.parse("Hello world").unwrap();
        assert!(parser.variables().is_empty());
    }

    #[test]
    fn test_empty_string_template() {
        let mut parser = TemplateParser::new();
        parser.parse("").unwrap();
        assert!(parser.variables().is_empty());
    }

    #[test]
    fn test_empty_variable_name_errors() {
        let mut parser = TemplateParser::new();
        let err = parser.parse("Hello {{}}").unwrap_err();
        assert!(matches!(err, Error::UnclosedVariable { .. }));
    }

    #[test]
    fn test_invalid_variable_name_errors() {
        let mut parser = TemplateParser::new();
        let err = parser.parse("Hello {{first name}}").unwrap_err();
        assert!(matches!(err, Error::InvalidVariableName(_)));
    }

    #[test]
    fn test_variable_name_with_underscore_is_valid() {
        let mut parser = TemplateParser::new();
        parser.parse("{{first_name}}").unwrap();
        assert_eq!(parser.variables(), &["first_name"]);
    }

    #[test]
    fn test_adjacent_variables() {
        let mut parser = TemplateParser::new();
        parser.parse("{{a}}{{b}}").unwrap();
        assert_eq!(parser.variables(), &["a", "b"]);
    }

    #[test]
    fn test_repeated_variable_is_captured_each_time() {
        let mut parser = TemplateParser::new();
        parser.parse("{{name}} and {{name}} again").unwrap();
        assert_eq!(parser.variables(), &["name", "name"]);
    }

    #[test]
    fn test_single_unmatched_brace_is_not_a_variable() {
        let mut parser = TemplateParser::new();
        parser.parse("just a { brace").unwrap();
        assert!(parser.variables().is_empty());
    }

    #[test]
    fn test_unicode_around_variable() {
        let mut parser = TemplateParser::new();
        parser.parse("こんにちは {{name}} さん 😀").unwrap();
        assert_eq!(parser.variables(), &["name"]);
    }
}
