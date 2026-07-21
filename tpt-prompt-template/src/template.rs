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

    pub fn render(&self, vars: &[( &str, &str)]) -> alloc::string::String {
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
