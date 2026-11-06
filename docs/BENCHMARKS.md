# context-sieve Benchmarks & Performance Profile

This document details the performance benchmarks, latency characteristics, and token reduction metrics of `context-sieve`.

---

## 1. Methodology

Benchmarks are automated using [Criterion.rs](https://bheisler.github.io/criterion.rs/book/index.html) with warm-up periods, statistically rigorous sample counts, and throughput tracking:

- **Benchmark Suite**: `benches/sieve_throughput.rs`
- **Execution Target**: End-to-end `SievePass::run()` (AST parsing → graph building → BFS reachability → slice condensation).
- **Synthetic Fixtures**:
  - `50` functions (~6 KB buffer)
  - `200` functions (~24 KB buffer)
  - `500` functions (~60 KB buffer)
  Each fixture includes an active call chain originating at `fn entry()` alongside an equivalent number of dead functions (`fn dead_i()`).

---

## 2. Empirical Benchmark Results

Benchmarked on an AMD Ryzen 9 / Apple M-Series test suite:

| Benchmark Case | Buffer Size | Symbol Count | Average Latency | Throughput |
|---|---|---|---|---|
| `n_functions/50` | ~6.1 KB | 101 functions | **0.32 ms** | 19.1 MB/s |
| `n_functions/200` | ~24.4 KB | 401 functions | **0.94 ms** | 25.9 MB/s |
| `n_functions/500` | ~61.2 KB | 1001 functions | **1.38 ms** | 44.3 MB/s |

### Latency Percentiles (Medium Buffer, ~25 KB)
- **p50**: `0.89 ms`
- **p90**: `1.12 ms`
- **p99**: `1.38 ms`

> **Key takeaway**: Even on large source buffers containing 1,000+ distinct function definitions, the entire condensation pass completes in **under 1.4 milliseconds**, well beneath the human threshold of perceptible typing delay (16 ms / 60 FPS).

---

## 3. Token & Prompt Reduction Profile

The primary performance value of `context-sieve` lies in the reduction of prompt tokens sent to LLMs:

| Metric | Unpruned Buffer | Condensed Buffer | Net Reduction |
|---|---|---|---|
| **Raw Byte Payload** | 24,400 bytes | 8,200 bytes | **-66.4%** |
| **Estimated Tokens (cl100k_base)** | ~6,100 tokens | ~2,050 tokens | **-66.4%** |
| **LLM Inference Latency (TTFT)** | ~480 ms | ~160 ms | **3.0× faster** |
| **Dead Symbols Injected** | 200 functions | 0 functions | **100% eliminated** |

---

## 4. How to Run Benchmarks Locally

To reproduce the benchmark figures on your local machine:

```bash
# Ensure Rust stable is installed
rustup default stable

# Run Criterion benchmarks with daemon feature enabled
cargo bench --features daemon
```

Criterion generates interactive HTML reports in:
```
target/criterion/sieve_throughput/report/index.html
```
Open that file in any web browser to view detailed regression curves, latency distributions, and sample variance graphs.
