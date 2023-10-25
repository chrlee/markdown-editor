// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use comrak::{parse_document, format_html, Arena, Options};
use comrak::nodes::{AstNode, NodeValue};


fn iter_nodes<'a, F>(node: &'a AstNode<'a>, f: &F)
        where F : Fn(&'a AstNode<'a>) {
        f(node);
        for c in node.children() {
            iter_nodes(c, f);
        }
    }

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn md_to_html(markdown: &str) -> String {
    // The returned nodes are created in the supplied Arena, and are bound by its lifetime.
    let arena = Arena::new();
    let options: &comrak::Options = &Options::default();
    let root = parse_document(
        &arena,
        markdown,
        options);

    let mut html = vec![];
    format_html(root, &Options::default(), &mut html).unwrap();

    String::from_utf8(html).unwrap()
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![md_to_html])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
