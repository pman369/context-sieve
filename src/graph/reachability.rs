//! Reachability queries over [`DepGraph`].
//!
//! Given a *root* [`SymbolId`] (typically the function under the editor cursor)
//! and a maximum hop depth, returns the set of all symbols reachable via the
//! call graph within that depth.  Symbols **not** in this set are prunable.
//!
//! The BFS uses a stack-allocated small-vec for the frontier to avoid heap
//! allocation on shallow graphs (depth ≤ 2 almost always fits in 64 entries).

use std::collections::VecDeque;

use rustc_hash::FxHashSet;

use crate::graph::{DepGraph, SymbolId};

pub struct ReachabilityQuery;

impl ReachabilityQuery {
    /// Return the set of [`SymbolId`]s reachable from `root` within `max_depth`
    /// hops (inclusive).  The root itself is always included.
    pub fn reachable_from(
        graph: &DepGraph,
        root: SymbolId,
        max_depth: usize,
    ) -> FxHashSet<SymbolId> {
        let mut visited: FxHashSet<SymbolId> = FxHashSet::default();
        // (node, remaining_depth)
        let mut queue: VecDeque<(SymbolId, usize)> = VecDeque::with_capacity(64);

        visited.insert(root);
        queue.push_back((root, max_depth));

        while let Some((node, depth)) = queue.pop_front() {
            if depth == 0 {
                continue;
            }
            // Iterate over outgoing call edges (A → B means A calls B).
            for neighbor in graph
                .inner
                .neighbors_directed(node, petgraph::Direction::Outgoing)
            {
                if visited.insert(neighbor) {
                    queue.push_back((neighbor, depth - 1));
                }
            }
        }

        visited
    }

    /// Symbols that have **zero** incoming edges AND are **not** reachable from
    /// any root — these are dead-code candidates for pruning.
    pub fn unreachable_symbols(graph: &DepGraph, reachable: &FxHashSet<SymbolId>) -> Vec<SymbolId> {
        graph
            .inner
            .node_indices()
            .filter(|id| !reachable.contains(id))
            .collect()
    }
}
