use criterion::{black_box, criterion_group, criterion_main, Criterion};
use metac::{Data, Evaluate, PartialParse, PartialParseOp};

// ---

criterion_main!(benches);
criterion_group!(benches, hello_world, partial_parser,);

// ---

fn hello_world(c: &mut Criterion) {
    struct Eval {}
    impl Evaluate<()> for Eval {
        fn evaluate(&mut self, args: &[Data]) -> () {
            assert_eq![
                &[
                    Data::Atom("Hello"),
                    Data::Command("World 1 2"),
                    Data::Atom("3")
                ],
                args
            ];
        }
    }

    let mut eval = Eval {};

    c.bench_function("hello_world", |b| {
        b.iter(|| {
            eval.interpret_single(black_box("Hello (World 1 2) 3"))
                .unwrap();
        })
    });
}

fn partial_parser(c: &mut Criterion) {
    let mut pp = PartialParse::default();
    c.bench_function("partial_parser", |b| {
        b.iter(|| {
            for byte in black_box(b"Hello (World 1 2) 3") {
                pp.parse_increment(*byte);
            }
            assert_eq![PartialParseOp::Ready, pp.parse_increment(black_box(b'\n'))];
        })
    });
}
