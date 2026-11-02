//! Integration tests for the sieve pipeline.

use context_sieve::{
    ast::{AstParser, Language},
    graph::builder::GraphBuilder,
    sieve::{SieveConfig, SievePass},
};

const RUST_FIXTURE: &str = r#"
fn entry_point() {
    helper_a();
}

fn helper_a() {
    helper_b();
}

fn helper_b() {
    // leaf
}

fn dead_code() {
    // Never called — should be pruned when cursor is in entry_point.
}

fn also_dead() {
    // Also unreachable.
    more_dead();
}

fn more_dead() {}
"#;

#[test]
fn parses_rust_symbols() {
    let mut parser = AstParser::new();
    let symbols = parser.parse(Language::Rust, RUST_FIXTURE.as_bytes());
    assert!(symbols.is_some(), "Rust parser returned None");
    let symbols = symbols.unwrap();
    let defs: Vec<_> = symbols.iter().filter(|s| s.is_definition()).collect();
    assert!(defs.len() >= 5, "expected ≥5 definitions, got {}", defs.len());
}

#[test]
fn graph_has_expected_edge_count() {
    let mut parser = AstParser::new();
    let symbols = parser.parse(Language::Rust, RUST_FIXTURE.as_bytes()).unwrap();
    let graph = GraphBuilder::build(&symbols, RUST_FIXTURE.as_bytes());
    // entry_point→helper_a, helper_a→helper_b = at least 2 edges.
    assert!(graph.edge_count() >= 2, "expected ≥2 call edges, got {}", graph.edge_count());
}

#[test]
fn sieve_reduces_payload() {
    let mut pass = SievePass::new();
    // Cursor at byte 5 — inside entry_point.
    let config = SieveConfig {
        language: Language::Rust,
        max_hop_depth: 2,
        cursor_byte_offset: 5,
    };
    let result = pass.run(RUST_FIXTURE.as_bytes(), &config);
    assert!(result.is_some(), "sieve returned None for valid Rust input");
    let result = result.unwrap();
    assert!(
        result.pruned_len < result.original_len,
        "expected pruning: pruned={} original={}",
        result.pruned_len,
        result.original_len
    );
    assert!(
        result.reduction_pct() > 10.0,
        "expected >10% reduction, got {:.1}%",
        result.reduction_pct()
    );
}

#[test]
fn sieve_preserves_reachable_code() {
    let mut pass = SievePass::new();
    let config = SieveConfig {
        language: Language::Rust,
        max_hop_depth: 2,
        cursor_byte_offset: 5,
    };
    let result = pass.run(RUST_FIXTURE.as_bytes(), &config).unwrap();
    let pruned_str = std::str::from_utf8(&result.pruned).unwrap();
    assert!(pruned_str.contains("entry_point"), "entry_point must be retained");
    assert!(pruned_str.contains("helper_a"),    "helper_a must be retained");
    assert!(pruned_str.contains("helper_b"),    "helper_b must be retained (2 hops)");
}

#[test]
fn condenser_handles_empty_prune_list() {
    use context_sieve::graph::DepGraph;
    use context_sieve::sieve::condenser::Condenser;
    let src = b"hello world";
    let graph = DepGraph::new();
    let result = Condenser::condense(src, &graph, &[]);
    assert_eq!(result, src);
}
