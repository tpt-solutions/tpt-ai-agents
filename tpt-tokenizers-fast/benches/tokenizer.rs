use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::collections::BTreeMap;
use tpt_tokenizers_fast::{BpeTokenizer, Vocabulary};

fn build_simple_tokenizer() -> BpeTokenizer {
    let mut vocab = Vocabulary::new();
    for (i, ch) in "abcdefghijklmnopqrstuvwxyz ".chars().enumerate() {
        vocab.insert(&ch.to_string(), i as u32);
    }
    BpeTokenizer::new(vocab, BTreeMap::new())
}

fn build_merge_tokenizer() -> BpeTokenizer {
    let mut vocab = Vocabulary::new();
    for (i, ch) in "abcdefghijklmnopqrstuvwxyz ".chars().enumerate() {
        vocab.insert(&ch.to_string(), i as u32);
    }
    // Add some merged tokens
    vocab.insert("th", 27);
    vocab.insert("he", 28);
    vocab.insert("in", 29);
    vocab.insert("er", 30);
    vocab.insert("an", 31);
    vocab.insert("re", 32);
    let mut merges = BTreeMap::new();
    merges.insert((19, 7), 27); // t+h -> th
    merges.insert((7, 4), 28); // h+e -> he
    merges.insert((8, 13), 29); // i+n -> in
    merges.insert((4, 17), 30); // e+r -> er
    merges.insert((0, 13), 31); // a+n -> an
    merges.insert((17, 4), 32); // r+e -> re
    BpeTokenizer::new(vocab, merges)
}

fn bench_encode_no_merges(c: &mut Criterion) {
    let tok = build_simple_tokenizer();
    c.bench_function("encode_no_merges", |b| {
        b.iter(|| tok.encode(black_box("hello world")).unwrap())
    });
}

fn bench_encode_with_merges(c: &mut Criterion) {
    let tok = build_merge_tokenizer();
    c.bench_function("encode_with_merges", |b| {
        b.iter(|| tok.encode(black_box("the weather")).unwrap())
    });
}

fn bench_encode_long_text(c: &mut Criterion) {
    let tok = build_simple_tokenizer();
    let text = "the quick brown fox jumps over the lazy dog ".repeat(100);
    c.bench_function("encode_long_text", |b| {
        b.iter(|| tok.encode(black_box(&text)).unwrap())
    });
}

fn bench_decode(c: &mut Criterion) {
    let tok = build_simple_tokenizer();
    let tokens: Vec<u32> = (0..26).collect();
    c.bench_function("decode_26_tokens", |b| {
        b.iter(|| tok.decode(black_box(&tokens)).unwrap())
    });
}

criterion_group!(
    benches,
    bench_encode_no_merges,
    bench_encode_with_merges,
    bench_encode_long_text,
    bench_decode,
);
criterion_main!(benches);
