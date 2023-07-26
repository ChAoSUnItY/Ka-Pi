use std::ffi::OsStr;
use std::fs;
use std::io::Cursor;

use criterion::{black_box, criterion_group, criterion_main, Criterion};

use ka_pi::parse::to_class;

fn parsing_classes(c: &mut Criterion) {
    let compiled_source_folder =
        fs::read_dir("compiled_source/out/production/compiled_source").unwrap();

    for compiled_source_path in compiled_source_folder {
        let compiled_source_path = compiled_source_path.as_ref().unwrap().path();
        let mut class_bytes = fs::read(&compiled_source_path).unwrap();

        c.bench_function(
            &format!(
                "parse {}",
                compiled_source_path
                    .file_name()
                    .and_then(OsStr::to_str)
                    .unwrap()
            ),
            |b| {
                b.iter(|| {
                    let mut cursor = Cursor::new(&mut class_bytes[..]);
                    let _class = to_class(black_box(&mut cursor)).unwrap();
                })
            },
        );
    }
}

criterion_group!(benches, parsing_classes);
criterion_main!(benches);
