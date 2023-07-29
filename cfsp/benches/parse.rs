use cfsp::parse::{to_class, ParsingOption};
use std::fs;
use std::io::Cursor;

use criterion::{criterion_group, criterion_main, BatchSize, BenchmarkId, Criterion, Throughput};

fn parsing_classes(c: &mut Criterion) {
    let mut g = c.benchmark_group("parse class");

    let compiled_source_folder =
        fs::read_dir("../compiled_source/out/production/compiled_source").unwrap();

    for entry in compiled_source_folder {
        let entry = entry.as_ref().unwrap().path();
        let class_name = entry.file_name().unwrap().to_str().unwrap();
        let class_bytes = fs::read(&entry).unwrap();

        g.throughput(Throughput::Bytes(class_bytes.len() as u64));
        g.bench_with_input(
            BenchmarkId::from_parameter(class_name),
            &class_bytes,
            |b, bytes| {
                b.iter_batched(
                    || Cursor::new(bytes),
                    |mut cursor| {
                        to_class(&mut cursor, ParsingOption::default().parse_attribute())
                            .expect("Parsing fails on benchmarking");
                    },
                    BatchSize::SmallInput,
                );
            },
        );

        g.bench_with_input(
            BenchmarkId::from_parameter(format!("{class_name} (No attribute parsing)")),
            &class_bytes,
            |b, bytes| {
                b.iter_batched(
                    || Cursor::new(bytes),
                    |mut cursor| {
                        to_class(&mut cursor, ParsingOption::default())
                            .expect("Parsing fails on benchmarking");
                    },
                    BatchSize::SmallInput,
                );
            },
        );
    }
}

criterion_group!(benches, parsing_classes);
criterion_main!(benches);
