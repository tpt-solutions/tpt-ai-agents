use criterion::{black_box, criterion_group, criterion_main, Criterion};
use tpt_llm_client_core::SseParser;

fn bench_feed_single_event(c: &mut Criterion) {
    c.bench_function("feed_single_event", |b| {
        let mut parser = SseParser::new();
        b.iter(|| parser.feed(black_box("data: {\"text\": \"hello\"}\n\n")))
    });
}

fn bench_feed_chunked_json(c: &mut Criterion) {
    let chunks = vec![
        "data: {\"choices\":[{\"delta\":{\"content\":\"Hello\"}}",
        "}",
        "data: {\"choices\":[{\"delta\":{\"content\":\" world\"}}",
        "}",
        "data: [DONE]\n\n",
    ];
    c.bench_function("feed_chunked_json", |b| {
        b.iter(|| {
            let mut parser = SseParser::new();
            for chunk in &chunks {
                parser.feed(black_box(chunk));
            }
        })
    });
}

fn bench_feed_many_events(c: &mut Criterion) {
    let input = "data: {\"text\": \"hello\"}\n\n".repeat(100);
    c.bench_function("feed_many_events", |b| {
        b.iter(|| {
            let mut parser = SseParser::new();
            parser.feed(black_box(&input));
        })
    });
}

fn bench_flush(c: &mut Criterion) {
    c.bench_function("flush", |b| {
        b.iter(|| {
            let mut parser = SseParser::new();
            parser.feed("data: partial");
            parser.flush()
        })
    });
}

criterion_group!(
    benches,
    bench_feed_single_event,
    bench_feed_chunked_json,
    bench_feed_many_events,
    bench_flush,
);
criterion_main!(benches);
