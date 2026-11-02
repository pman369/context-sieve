//! The Sieve pass — the core condensation algorithm.
//!
//! [`SievePass`] ties together AST parsing, graph construction, and
//! reachability analysis to produce a pruned byte slice (or token estimate)
//! from a raw source buffer.
//!
//! ## Pipeline
//!
//! ```text
//! raw source bytes
//!       │
//!       ▼  AstParser::parse()
//! Vec<Symbol>
//!       │
//!       ▼  GraphBuilder::build()
//! DepGraph
//!       │
//!       ▼  ReachabilityQuery::reachable_from(cursor_symbol, depth)
//! FxHashSet<SymbolId>
//!       │
//!       ▼  Condenser::condense()
//! pruned Vec<u8>   ← safe to inject into LLM prompt
//! ```

pub mod condenser;

pub use condenser::Condenser;

use crate::ast::{AstParser, Language};
use crate::graph::{builder::GraphBuilder, reachability::ReachabilityQuery};

/// Configuration for a sieve pass.
#[derive(Debug, Clone)]
pub struct SieveConfig {
    /// Language of the source buffer.
    pub language: Language,
    /// Maximum call-graph hop depth for reachability analysis.
    pub max_hop_depth: usize,
    /// Byte offset of the user's cursor inside the source buffer.
    /// The symbol enclosing this offset is used as the reachability root.
    pub cursor_byte_offset: usize,
}

impl Default for SieveConfig {
    fn default() -> Self {
        Self {
            language: Language::Rust,
            max_hop_depth: 2,
            cursor_byte_offset: 0,
        }
    }
}

/// Result of a sieve pass.
pub struct SieveResult {
    /// The pruned source bytes (still valid UTF-8 if the input was).
    pub pruned: Vec<u8>,
    /// Original source length in bytes.
    pub original_len: usize,
    /// Pruned source length in bytes.
    pub pruned_len: usize,
    /// Number of symbols that were removed.
    pub pruned_symbol_count: usize,
}

impl SieveResult {
    /// Approximate reduction percentage.
    pub fn reduction_pct(&self) -> f64 {
        if self.original_len == 0 {
            return 0.0;
        }
        (1.0 - self.pruned_len as f64 / self.original_len as f64) * 100.0
    }
}

/// Stateful sieve executor.  Hold one per thread; not `Sync`.
pub struct SievePass {
    parser: AstParser,
}

impl SievePass {
    pub fn new() -> Self {
        Self { parser: AstParser::new() }
    }

    /// Execute the full sieve pipeline over `source`.
    ///
    /// Returns `None` if the language is unsupported or parsing fails,
    /// in which case callers should forward the original buffer unchanged.
    pub fn run(&mut self, source: &[u8], config: &SieveConfig) -> Option<SieveResult> {
        let original_len = source.len();

        // 1. Parse AST → symbols.
        let symbols = self.parser.parse(config.language, source)?;

        // 2. Build dependency graph.
        let graph = GraphBuilder::build(&symbols, source);

        // 3. Find cursor-enclosing symbol.
        let root = symbols.iter().enumerate().find(|(_, sym)| {
            sym.is_definition()
                && sym.body.start as usize <= config.cursor_byte_offset
                && config.cursor_byte_offset <= sym.body.end as usize
        });

        let reachable = if let Some((idx, _sym)) = root {
            // Map symbol index → graph NodeIndex (graph nodes are inserted in
            // definition order, so the first `idx` definition nodes align).
            use petgraph::graph::NodeIndex;
            let def_idx = symbols[..=idx]
                .iter()
                .filter(|s| s.is_definition())
                .count()
                .saturating_sub(1);
            let node_id = NodeIndex::new(def_idx);
            ReachabilityQuery::reachable_from(&graph, node_id, config.max_hop_depth)
        } else {
            // No cursor symbol: keep everything.
            use rustc_hash::FxHashSet;
            graph.inner.node_indices().collect::<FxHashSet<_>>()
        };

        // 4. Condense: remove unreachable symbol bodies.
        let unreachable = ReachabilityQuery::unreachable_symbols(&graph, &reachable);
        let pruned_symbol_count = unreachable.len();

        let pruned = Condenser::condense(source, &graph, &unreachable);
        let pruned_len = pruned.len();

        Some(SieveResult {
            pruned,
            original_len,
            pruned_len,
            pruned_symbol_count,
        })
    }
}

impl Default for SievePass {
    fn default() -> Self { Self::new() }
}
