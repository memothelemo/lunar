use criterion::{black_box, criterion_group, criterion_main, Criterion};
use lunar_tokenizer::tokenize as tokenizer;

const T_SOURCE: &str = include_str!("./t.lun");

fn tokenize(criterion: &mut Criterion) {
    criterion.bench_function("tokenize t", |b| b.iter(|| tokenizer(black_box(T_SOURCE))));
}

fn parse(criterion: &mut Criterion) {
    use lunar_parser::Parser;
    let tokens = lunar_ast::filter_non_trivia_tokens(tokenizer(T_SOURCE).unwrap());

    criterion.bench_function("parse t", move |b| {
        b.iter(|| {
            let state = lunar_parser::ParseState::new(&tokens);
            lunar_parser::ParseBlock.parse(&state)
        })
    });
}

criterion_group! {
    name = benches;
    config = Criterion::default().sample_size(20);
    targets = tokenize, parse
}

criterion_main!(benches);
