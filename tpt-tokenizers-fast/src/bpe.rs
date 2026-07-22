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
        // Start from per-character symbols, then greedily apply the learned
        // merges in priority order (lowest merged-id first == earliest-learned
        // merge first), same strategy as the reference BPE algorithm.
        let mut symbols: alloc::vec::Vec<TokenId> = alloc::vec::Vec::new();
        for ch in text.chars() {
            let char_str = alloc::string::String::from(ch);
            match self.vocab.get_id(&char_str) {
                Some(id) => symbols.push(id),
                None => return Err(Error::UnknownToken(char_str)),
            }
        }

        loop {
            // Find the adjacent pair with the lowest merge rank (earliest-learned merge).
            let mut best: Option<(usize, TokenId)> = None;
            for i in 0..symbols.len().saturating_sub(1) {
                if let Some(&merged_id) = self.merges.get(&(symbols[i], symbols[i + 1])) {
                    let better = match best {
                        Some((_, current_best)) => merged_id < current_best,
                        None => true,
                    };
                    if better {
                        best = Some((i, merged_id));
                    }
                }
            }

            match best {
                Some((i, merged_id)) => {
                    symbols[i] = merged_id;
                    symbols.remove(i + 1);
                }
                None => break,
            }
        }

        let mut tokens = alloc::vec::Vec::with_capacity(symbols.len());
        for id in symbols {
            let text = self.vocab.get_token(id).ok_or_else(|| {
                Error::Encoding(alloc::format!("no vocab entry for merged token id {id}"))
            })?;
            tokens.push(Token::new(id, text));
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
    fn test_encode_applies_merges() {
        // "l" + "o" merges into "lo" (id 5), then "lo" + " " does not merge further.
        let mut vocab = simple_vocab();
        vocab.insert("lo", 5);
        let mut merges = BTreeMap::new();
        merges.insert((2, 3), 5); // (l, o) -> lo

        let tok = BpeTokenizer::new(vocab, merges);
        let tokens = tok.encode("lo").unwrap();
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0].id, 5);
        assert_eq!(tokens[0].text, "lo");
    }

    #[test]
    fn test_encode_applies_merges_in_priority_order() {
        // Merges are applied in lowest-merged-id-first order: (h,e)->5 is
        // learned before (e,l)->6, so "hel" should merge to "he"+"l", not "h"+"el".
        let mut vocab = simple_vocab();
        vocab.insert("he", 5);
        vocab.insert("el", 6);
        let mut merges = BTreeMap::new();
        merges.insert((0, 1), 5); // (h, e) -> he
        merges.insert((1, 2), 6); // (e, l) -> el

        let tok = BpeTokenizer::new(vocab, merges);
        let tokens = tok.encode("hel").unwrap();
        assert_eq!(tokens.len(), 2);
        assert_eq!(tokens[0].text, "he");
        assert_eq!(tokens[1].text, "l");
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
