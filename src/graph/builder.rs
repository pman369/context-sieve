//! Builds a [`DepGraph`] from a list of [`Symbol`]s.
//!
//! Two-pass algorithm:
//! 1. **Definition pass** — insert every definition symbol as a graph node and
//!    populate the `name_index`.
//! 2. **Call-site pass** — for each `CallSite` symbol, look up the callee by
//!    name hash and add a directed edge from the enclosing definition.

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

use crate::ast::symbol::{Symbol, SymbolKind};
use crate::graph::{DepGraph, SymbolId, SymbolMeta};

pub struct GraphBuilder;

impl GraphBuilder {
    /// Consume a symbol list and produce a populated [`DepGraph`].
    ///
    /// `source` is the original byte buffer; used only to hash name slices.
    pub fn build(symbols: &[Symbol], source: &[u8]) -> DepGraph {
        let mut graph = DepGraph::new();

        // ── Pass 1: Insert definitions ──────────────────────────────────────
        let mut def_nodes: Vec<(SymbolId, Symbol)> = Vec::with_capacity(symbols.len());

        for &sym in symbols {
            if sym.is_definition() {
                let name_bytes = sym.name.slice(source);
                let h = hash_bytes(name_bytes);
                let meta = SymbolMeta {
                    name_start:    sym.name.start,
                    name_end:      sym.name.end,
                    body_start:    sym.body.start,
                    body_end:      sym.body.end,
                    is_definition: true,
                };
                let id = graph.inner.add_node(meta);
                graph.name_index.insert(h, id);
                def_nodes.push((id, sym));
            }
        }

        // ── Pass 2: Wire call edges ─────────────────────────────────────────
        // For simplicity, a call site's "enclosing definition" is the most
        // recently seen definition whose body span contains the call site.
        for &sym in symbols {
            if sym.kind != SymbolKind::CallSite {
                continue;
            }

            // Find enclosing definition (last def that starts before this call).
            let caller_id = def_nodes
                .iter()
                .rev()
                .find(|(_, def)| {
                    def.body.start <= sym.body.start && sym.body.end <= def.body.end
                })
                .map(|(id, _)| *id);

            let callee_name = sym.name.slice(source);
            // Strip any path prefix (e.g. `self.foo` → `foo`).
            let leaf = leaf_name(callee_name);
            let h = hash_bytes(leaf);

            if let (Some(caller), Some(&callee)) =
                (caller_id, graph.name_index.get(&h))
            {
                // Avoid self-loops.
                if caller != callee {
                    graph.inner.add_edge(caller, callee, ());
                }
            }
        }

        graph
    }
}

/// Strip method-call prefix: `b"self.foo"` → `b"foo"`.
fn leaf_name(bytes: &[u8]) -> &[u8] {
    match bytes.iter().rposition(|&b| b == b'.' || b == b':') {
        Some(pos) => &bytes[pos + 1..],
        None      => bytes,
    }
}

fn hash_bytes(b: &[u8]) -> u64 {
    let mut h = DefaultHasher::new();
    b.hash(&mut h);
    h.finish()
}
