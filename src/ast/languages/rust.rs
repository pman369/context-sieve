//! Rust language backend — native only; excluded from WASM builds.
//!
//! This file is compiled only when `target_arch != "wasm32"`.
#![cfg(not(target_arch = "wasm32"))]
//!
//!
//! Walks the Tree-sitter concrete syntax tree for Rust source and emits
//! [`Symbol`]s for function definitions, struct/enum/trait declarations,
//! impl blocks, use declarations, and call expressions.
//!
//! The visitor is a zero-allocation depth-first cursor walk — no intermediate
//! Vecs are created for child lists.

use tree_sitter::{Node, Parser, TreeCursor};

use crate::ast::{
    languages::LanguageParser,
    symbol::{Span, Symbol, SymbolKind},
};

pub struct RustParser;

impl LanguageParser for RustParser {
    fn parse(parser: &mut Parser, source: &[u8]) -> Option<Vec<Symbol>> {
        parser.set_language(&tree_sitter_rust::language()).ok()?;

        let tree = parser.parse(source, None)?;
        let mut symbols = Vec::new();
        let mut cursor = tree.walk();

        walk_rust(&mut cursor, source, 0, &mut symbols);
        Some(symbols)
    }
}

/// Depth-first CST walk; `depth` tracks nesting level inside the file.
fn walk_rust(cursor: &mut TreeCursor, source: &[u8], depth: u8, out: &mut Vec<Symbol>) {
    loop {
        let node = cursor.node();
        visit_rust_node(node, source, depth, out);

        if cursor.goto_first_child() {
            walk_rust(cursor, source, depth.saturating_add(1), out);
            cursor.goto_parent();
        }

        if !cursor.goto_next_sibling() {
            break;
        }
    }
}

fn visit_rust_node(node: Node, _source: &[u8], depth: u8, out: &mut Vec<Symbol>) {
    let kind = node.kind();
    let body = span_of(&node);

    match kind {
        // --- Definitions ---
        "function_item" => {
            if let Some(name_node) = node.child_by_field_name("name") {
                out.push(Symbol::new(
                    span_of(&name_node),
                    body,
                    SymbolKind::Function,
                    depth,
                ));
            }
        }
        "struct_item" => {
            if let Some(name_node) = node.child_by_field_name("name") {
                out.push(Symbol::new(
                    span_of(&name_node),
                    body,
                    SymbolKind::Struct,
                    depth,
                ));
            }
        }
        "enum_item" => {
            if let Some(name_node) = node.child_by_field_name("name") {
                out.push(Symbol::new(
                    span_of(&name_node),
                    body,
                    SymbolKind::Enum,
                    depth,
                ));
            }
        }
        "trait_item" => {
            if let Some(name_node) = node.child_by_field_name("name") {
                out.push(Symbol::new(
                    span_of(&name_node),
                    body,
                    SymbolKind::Trait,
                    depth,
                ));
            }
        }
        "impl_item" => {
            // impl blocks: name is the type being implemented.
            if let Some(type_node) = node.child_by_field_name("type") {
                out.push(Symbol::new(
                    span_of(&type_node),
                    body,
                    SymbolKind::Impl,
                    depth,
                ));
            }
        }
        "mod_item" => {
            if let Some(name_node) = node.child_by_field_name("name") {
                out.push(Symbol::new(
                    span_of(&name_node),
                    body,
                    SymbolKind::Module,
                    depth,
                ));
            }
        }
        "use_declaration" => {
            // Record the entire use path as the name span.
            out.push(Symbol::new(body, body, SymbolKind::Import, depth));
        }
        "type_item" => {
            if let Some(name_node) = node.child_by_field_name("name") {
                out.push(Symbol::new(
                    span_of(&name_node),
                    body,
                    SymbolKind::TypeAlias,
                    depth,
                ));
            }
        }
        "const_item" | "static_item" => {
            if let Some(name_node) = node.child_by_field_name("name") {
                out.push(Symbol::new(
                    span_of(&name_node),
                    body,
                    SymbolKind::Constant,
                    depth,
                ));
            }
        }
        // --- Call sites ---
        "call_expression" => {
            if let Some(func_node) = node.child_by_field_name("function") {
                out.push(Symbol::new(
                    span_of(&func_node),
                    body,
                    SymbolKind::CallSite,
                    depth,
                ));
            }
        }
        _ => {}
    }
}

#[inline]
fn span_of(node: &Node) -> Span {
    Span::new(node.start_byte() as u32, node.end_byte() as u32)
}
