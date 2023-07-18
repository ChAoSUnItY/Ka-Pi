use criterion::{black_box, criterion_group, criterion_main, Criterion};
use ka_pi::parse::to_class;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("parse main", |b| b.iter(|| to_class(black_box(include_bytes!(
        "../compiled_source/out/production/compiled_source/Main.class"
    )))));

    c.bench_function("parse enum", |b| b.iter(|| to_class(black_box(include_bytes!(
        "../compiled_source/out/production/compiled_source/Enum.class"
    )))));

    c.bench_function("parse record", |b| b.iter(|| to_class(black_box(include_bytes!(
        "../compiled_source/out/production/compiled_source/Record.class"
    )))));

    c.bench_function("parse visible annotation", |b| b.iter(|| to_class(black_box(include_bytes!(
        "../compiled_source/out/production/compiled_source/VisibleAnnotation.class"
    )))));

    c.bench_function("parse invisible annotation", |b| b.iter(|| to_class(black_box(include_bytes!(
        "../compiled_source/out/production/compiled_source/InvisibleAnnotation.class"
    )))));

    c.bench_function("parse annotation target", |b| b.iter(|| to_class(black_box(include_bytes!(
        "../compiled_source/out/production/compiled_source/AnnotationTarget.class"
    )))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
