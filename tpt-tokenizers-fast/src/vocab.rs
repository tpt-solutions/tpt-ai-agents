use alloc::collections::BTreeMap;

use crate::TokenId;

/// Vocabulary mapping tokens to IDs.
pub struct Vocabulary {
    token_to_id: BTreeMap<alloc::string::String, TokenId>,
    id_to_token: BTreeMap<TokenId, alloc::string::String>,
}

impl Vocabulary {
    pub fn new() -> Self {
        Self {
            token_to_id: BTreeMap::new(),
            id_to_token: BTreeMap::new(),
        }
    }

    pub fn insert(&mut self, token: &str, id: TokenId) {
        self.token_to_id
            .insert(alloc::string::String::from(token), id);
        self.id_to_token
            .insert(id, alloc::string::String::from(token));
    }

    pub fn get_id(&self, token: &str) -> Option<TokenId> {
        self.token_to_id.get(token).copied()
    }

    pub fn get_token(&self, id: TokenId) -> Option<&str> {
        self.id_to_token.get(&id).map(|s| s.as_str())
    }

    pub fn size(&self) -> usize {
        self.token_to_id.len()
    }
}

impl Default for Vocabulary {
    fn default() -> Self {
        Self::new()
    }
}
