use crate::{Error, Token, TokenId, Vocabulary};
use alloc::collections::BTreeMap;

/// BPE tokenizer.
pub struct BpeTokenizer {
    vocab: Vocabulary,
    merges: BTreeMap<(TokenId, TokenId), TokenId>,
}

impl BpeTokenizer {
    pub fn new(vocab: Vocabulary, merges: BTreeMap<(TokenId, TokenId), TokenId>) -> Self {
        Self { vocab, merges }
    }

    pub fn encode(&self, text: &str) -> core::result::Result<alloc::vec::Vec<Token>, Error> {
        let mut tokens = alloc::vec::Vec::new();
        for ch in text.chars() {
            let char_str = alloc::string::String::from(ch);
            if let Some(id) = self.vocab.get_id(&char_str) {
                tokens.push(Token::new(id, &char_str));
            } else {
                return Err(Error::UnknownToken(char_str));
            }
        }
        Ok(tokens)
    }

    pub fn decode(&self, tokens: &[TokenId]) -> core::result::Result<alloc::string::String, Error> {
        let mut result = alloc::string::String::new();
        for &id in tokens {
            if let Some(token) = self.vocab.get_token(id) {
                result.push_str(token);
            } else {
                return Err(Error::Decoding(alloc::format!("unknown token id: {id}")));
            }
        }
        Ok(result)
    }

    pub fn vocab_size(&self) -> usize {
        self.vocab.size()
    }
}
