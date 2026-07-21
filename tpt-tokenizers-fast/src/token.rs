/// Token ID type.
pub type TokenId = u32;

/// A decoded token.
#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub id: TokenId,
    pub text: alloc::string::String,
}

impl Token {
    pub fn new(id: TokenId, text: &str) -> Self {
        Self {
            id,
            text: alloc::string::String::from(text),
        }
    }
}
