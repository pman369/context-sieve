//! Symbol dependency graph.
//!
//! Wraps a `petgraph::DiGraph` where every node is a [`SymbolId`] (a compact
//! `u32` handle) and every directed edge represents a call or use relationship:
//! `A → B` means symbol A *uses/calls* symbol B.
//!
//! The graph is built once per buffer parse and lives in a `HashMap` keyed by
//! (file URI, symbol name bytes).  Reachability queries then walk from a
//! *cursor origin* outward up to a configurable hop depth.

pub mod builder;
pub mod reachability;

pub use builder::GraphBuilder;
pub use reachability::ReachabilityQuery;

use petgraph::graph::{DiGraph, NodeIndex};
use rustc_hash::FxHashMap;

/// Opaque handle to a node in the dependency graph.
pub type SymbolId = NodeIndex<u32>;

/// The live dependency graph for a single parsed buffer.
#[derive(Debug, Default)]
pub struct DepGraph {
    /// Directed call graph: edge A→B means A uses B.
    pub(crate) inner: DiGraph<SymbolMeta, (), u32>,
    /// Map from (name_bytes hash) → NodeIndex for O(1) name lookup.
    pub(crate) name_index: FxHashMap<u64, SymbolId>,
}

impl DepGraph {
    pub fn new() -> Self {
        Self::default()
    }

    /// Number of symbols in the graph.
    pub fn symbol_count(&self) -> usize {
        self.inner.node_count()
    }

    /// Number of call edges in the graph.
    pub fn edge_count(&self) -> usize {
        self.inner.edge_count()
    }
}

/// Metadata stored at each graph node.
#[derive(Debug, Clone)]
pub struct SymbolMeta {
    /// Byte offset of the symbol name in the original source buffer.
    pub name_start: u32,
    pub name_end: u32,
    /// Byte offset of the complete node body.
    pub body_start: u32,
    pub body_end: u32,
    /// Whether the symbol is a definition (vs. call site / import).
    pub is_definition: bool,
}
