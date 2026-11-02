//! Buffer condensation — remove unreachable symbol bodies from source bytes.
//!
//! The [`Condenser`] walks the list of prunable [`SymbolId`]s, collects their
//! body byte ranges, merges overlapping/adjacent ranges, and re-assembles the
//! source as a new `Vec<u8>` by copying only the *kept* slices.
//!
//! This is O(n log n) in the number of pruned symbols (due to the sort) and
//! O(source_len) in memory — a single allocation for the output buffer.

use crate::graph::{DepGraph, SymbolId};

pub struct Condenser;

impl Condenser {
    /// Return a new byte buffer with all `pruned` symbols' bodies removed.
    pub fn condense(source: &[u8], graph: &DepGraph, pruned: &[SymbolId]) -> Vec<u8> {
        if pruned.is_empty() {
            return source.to_vec();
        }

        // Collect body byte ranges of pruned symbols.
        let mut ranges: Vec<(u32, u32)> = pruned
            .iter()
            .filter_map(|&id| {
                let meta = graph.inner.node_weight(id)?;
                Some((meta.body_start, meta.body_end))
            })
            .collect();

        // Sort by start offset.
        ranges.sort_unstable_by_key(|&(s, _)| s);

        // Merge overlapping / adjacent ranges.
        let merged = merge_ranges(ranges);

        // Re-assemble output: copy source slices that are *not* pruned.
        let output_cap = source.len(); // upper bound
        let mut out = Vec::with_capacity(output_cap);
        let mut cursor: usize = 0;

        for (prune_start, prune_end) in merged {
            let ps = prune_start as usize;
            let pe = prune_end as usize;

            if ps > cursor {
                out.extend_from_slice(&source[cursor..ps]);
            }
            cursor = pe;
        }

        // Append the tail after the last pruned range.
        if cursor < source.len() {
            out.extend_from_slice(&source[cursor..]);
        }

        out
    }
}

/// Merge a **sorted** list of potentially overlapping byte ranges.
fn merge_ranges(sorted: Vec<(u32, u32)>) -> Vec<(u32, u32)> {
    let mut merged: Vec<(u32, u32)> = Vec::with_capacity(sorted.len());

    for (s, e) in sorted {
        match merged.last_mut() {
            Some(last) if s <= last.1 => {
                last.1 = last.1.max(e);
            }
            _ => merged.push((s, e)),
        }
    }

    merged
}
