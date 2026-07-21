use crate::{Error, Token, TokenId, Vocabulary};
use alloc::collections::BTreeMap;

/// BPE tokenizer.
pub struct BpeTokenizer {
    vocab: Vocabulary,
    #[allow(dead_code)]
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

#[cfg(test)]
mod tests {
    use super::*;

    fn simple_vocab() -> Vocabulary {
        let mut v = Vocabulary::new();
        v.insert("h", 0);
        v.insert("e", 1);
        v.insert("l", 2);
        v.insert("o", 3);
        v.insert(" ", 4);
        v
    }

    #[test]
    fn test_encode() {
        let tok = BpeTokenizer::new(simple_vocab(), BTreeMap::new());
        let tokens = tok.encode("hel").unwrap();
        assert_eq!(tokens.len(), 3);
        assert_eq!(tokens[0].text, "h");
        assert_eq!(tokens[1].text, "e");
        assert_eq!(tokens[2].text, "l");
    }

    #[test]
    fn test_encode_unknown_token() {
        let tok = BpeTokenizer::new(simple_vocab(), BTreeMap::new());
        assert!(tok.encode("x").is_err());
    }

    #[test]
    fn test_decode() {
        let tok = BpeTokenizer::new(simple_vocab(), BTreeMap::new());
        let decoded = tok.decode(&[0, 1, 2]).unwrap();
        assert_eq!(decoded, "hel");
    }

    #[test]
    fn test_decode_unknown_id() {
        let tok = BpeTokenizer::new(simple_vocab(), BTreeMap::new());
        assert!(tok.decode(&[99]).is_err());
    }

    #[test]
    fn test_vocab_size() {
        let tok = BpeTokenizer::new(simple_vocab(), BTreeMap::new());
        assert_eq!(tok.vocab_size(), 5);
    }
}
