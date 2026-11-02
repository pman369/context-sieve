//! Python language backend.
//!
//! Extracts function/class definitions, import statements, and call
//! expressions from Python source via the tree-sitter-python grammar.

use tree_sitter::{Node, Parser, TreeCursor};

use crate::ast::{
    languages::LanguageParser,
    symbol::{Span, Symbol, SymbolKind},
};

pub struct PythonParser;

impl LanguageParser for PythonParser {
    fn parse(parser: &mut Parser, source: &[u8]) -> Option<Vec<Symbol>> {
        parser
            .set_language(&tree_sitter_python::language().into())
            .ok()?;

        let tree = parser.parse(source, None)?;
        let mut symbols = Vec::new();
        let mut cursor = tree.walk();

        walk_python(&mut cursor, source, 0, &mut symbols);
        Some(symbols)
    }
}

fn walk_python(cursor: &mut TreeCursor, source: &[u8], depth: u8, out: &mut Vec<Symbol>) {
    loop {
        let node = cursor.node();
        visit_python_node(node, source, depth, out);

        if cursor.goto_first_child() {
            walk_python(cursor, source, depth.saturating_add(1), out);
            cursor.goto_parent();
        }

        if !cursor.goto_next_sibling() {
            break;
        }
    }
}

fn visit_python_node(node: Node, _source: &[u8], depth: u8, out: &mut Vec<Symbol>) {
    let kind = node.kind();
    let body = span_of(&node);

    match kind {
        "function_definition" | "async_function_definition" => {
            if let Some(name_node) = node.child_by_field_name("name") {
                out.push(Symbol::new(span_of(&name_node), body, SymbolKind::Function, depth));
            }
        }
        "class_definition" => {
            if let Some(name_node) = node.child_by_field_name("name") {
                out.push(Symbol::new(span_of(&name_node), body, SymbolKind::Struct, depth));
            }
        }
        "import_statement" | "import_from_statement" => {
            out.push(Symbol::new(body, body, SymbolKind::Import, depth));
        }
        "call" => {
            if let Some(func_node) = node.child_by_field_name("function") {
                out.push(Symbol::new(span_of(&func_node), body, SymbolKind::CallSite, depth));
            }
        }
        _ => {}
    }
}

#[inline]
fn span_of(node: &Node) -> Span {
    Span::new(node.start_byte() as u32, node.end_byte() as u32)
}
