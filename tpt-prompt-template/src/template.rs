use crate::parser::TemplateParser;

/// A validated prompt template.
pub struct PromptTemplate {
    template: alloc::string::String,
    variables: alloc::vec::Vec<alloc::string::String>,
}

impl PromptTemplate {
    pub fn new(template: &str) -> core::result::Result<Self, crate::Error> {
        let mut parser = TemplateParser::new();
        parser.parse(template)?;
        Ok(Self {
            template: alloc::string::String::from(template),
            variables: parser.variables().to_vec(),
        })
    }

    pub fn render(&self, vars: &[(&str, &str)]) -> alloc::string::String {
        let mut result = self.template.clone();
        for (key, value) in vars {
            let placeholder = alloc::format!("{{{{{key}}}}}");
            result = result.replace(&placeholder, value);
        }
        result
    }

    pub fn variables(&self) -> &[alloc::string::String] {
        &self.variables
    }

    pub fn as_str(&self) -> &str {
        &self.template
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_render_substitutes_all_variables() {
        let template = PromptTemplate::new("Hello, {{name}}! Topic: {{topic}}.").unwrap();
        let rendered = template.render(&[("name", "Ada"), ("topic", "Rust")]);
        assert_eq!(rendered, "Hello, Ada! Topic: Rust.");
    }

    #[test]
    fn test_render_leaves_missing_variable_unresolved() {
        let template = PromptTemplate::new("Hello, {{name}}!").unwrap();
        let rendered = template.render(&[]);
        assert_eq!(rendered, "Hello, {{name}}!");
    }

    #[test]
    fn test_render_ignores_unknown_extra_vars() {
        let template = PromptTemplate::new("Hello, {{name}}!").unwrap();
        let rendered = template.render(&[("name", "Ada"), ("unused", "x")]);
        assert_eq!(rendered, "Hello, Ada!");
    }

    #[test]
    fn test_render_with_no_variables_returns_template_unchanged() {
        let template = PromptTemplate::new("Just plain text.").unwrap();
        assert_eq!(template.render(&[]), "Just plain text.");
    }

    #[test]
    fn test_new_rejects_malformed_template() {
        assert!(PromptTemplate::new("Hello {{}}").is_err());
    }

    #[test]
    fn test_variables_reports_declared_names() {
        let template = PromptTemplate::new("{{a}} and {{b}}").unwrap();
        assert_eq!(template.variables(), &["a", "b"]);
    }

    #[test]
    fn test_render_substitutes_unicode_values() {
        let template = PromptTemplate::new("Hi {{name}}").unwrap();
        let rendered = template.render(&[("name", "日本語 😀")]);
        assert_eq!(rendered, "Hi 日本語 😀");
    }
}
