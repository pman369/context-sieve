//! Criterion benchmark: end-to-end sieve throughput on a synthetic buffer.

use context_sieve::{
    ast::Language,
    sieve::{SieveConfig, SievePass},
};
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};

/// Generate a synthetic Rust source file with `n` functions.
fn generate_fixture(n: usize) -> String {
    let mut src = String::with_capacity(n * 120);
    // Entry point calls the first helper.
    src.push_str("fn entry() {\n    helper_0();\n}\n\n");
    for i in 0..n {
        let next = i + 1;
        if next < n {
            src.push_str(&format!("fn helper_{i}() {{\n    helper_{next}();\n}}\n\n"));
        } else {
            src.push_str(&format!("fn helper_{i}() {{\n    // leaf\n}}\n\n"));
        }
        // Add a dead helper for every real helper.
        src.push_str(&format!("fn dead_{i}() {{\n    // never called\n}}\n\n"));
    }
    src
}

fn bench_sieve(c: &mut Criterion) {
    let mut group = c.benchmark_group("sieve_throughput");

    for size in [50usize, 200, 500] {
        let fixture = generate_fixture(size);
        let bytes = fixture.as_bytes();

        group.throughput(Throughput::Bytes(bytes.len() as u64));
        group.bench_with_input(BenchmarkId::new("n_functions", size), &fixture, |b, src| {
            let mut pass = SievePass::new();
            let config = SieveConfig {
                language: Language::Rust,
                max_hop_depth: 2,
                cursor_byte_offset: 5,
            };
            b.iter(|| pass.run(src.as_bytes(), &config));
        });
    }

    group.finish();
}

criterion_group!(benches, bench_sieve);
criterion_main!(benches);
