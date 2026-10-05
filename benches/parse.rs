#[cfg(not(feature = "async"))]
use std::fs::File;
#[cfg(not(feature = "async"))]
use std::io::{Cursor, Read};
use std::sync::{Arc, atomic::AtomicBool};

#[cfg(not(feature = "async"))]
use criterion::{Criterion, criterion_group, criterion_main};
#[cfg(not(feature = "async"))]
use lopdf::Document;

#[cfg(not(feature = "async"))]
fn bench_load(c: &mut Criterion) {
    let mut buffer = Vec::new();
    File::open("assets/example.pdf")
        .unwrap()
        .read_to_end(&mut buffer)
        .unwrap();

    c.bench_function("load_example_pdf", |b| {
        b.iter(|| {
            let stop = Arc::new(AtomicBool::new(false));
            Document::load_from(Cursor::new(&buffer), stop).unwrap();
        });
    });
}

#[cfg(not(feature = "async"))]
fn bench_load_incremental_pdf(c: &mut Criterion) {
    let mut buffer = Vec::new();
    File::open("assets/Incremental.pdf")
        .unwrap()
        .read_to_end(&mut buffer)
        .unwrap();

    c.bench_function("load_incremental_pdf", |b| {
        b.iter(|| {
            let stop = Arc::new(AtomicBool::new(false));
            Document::load_from(Cursor::new(&buffer), stop).unwrap();
        });
    });
}

#[cfg(not(feature = "async"))]
criterion_group!(benches, bench_load, bench_load_incremental_pdf);
#[cfg(not(feature = "async"))]
criterion_main!(benches);

#[cfg(feature = "async")]
fn main() {}
