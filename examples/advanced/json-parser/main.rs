mod ast;
mod grammar;

use grammar::JsonValue;
use grammar::json;
use ptree::print_tree;

fn main() {
    let sample_json = r#"{
        "project": "semasia",
        "version": 0.1,
        "is_active": true,
        "features": [
            "LALR(1) parsing",
            "compile-time tables",
            "zero-copy ownership",
            "EBNF support"
        ],
        "metadata": {
            "license": "MIT OR Apache-2.0",
            "repository": null,
            "escaped_text": "quote: \"hello\", newline: \n, unicode: \u0041"
        },
        "empty_list": [],
        "empty_map": {}
    }"#;

    println!("=== Parsing Sample JSON ===");
    match json::Parser::lex_parse(sample_json) {
        Ok(parsed) => {
            println!("\n--- Pretty-Printed JSON ---");
            println!("{parsed}");

            println!("\n--- AST Tree View (ptree) ---");
            print_tree(&parsed.build_tree()).expect("could not print tree");

            println!("\n--- Pattern Matching Parsed JSON ---");
            if let JsonValue::Object(map) = &parsed {
                if let Some(JsonValue::String(project)) = map.get("\"project\"") {
                    println!("Project name: {project}");
                }
                if let Some(JsonValue::Bool(val)) = map.get("\"is_active\"") {
                    println!("Is active: {val}");
                }
                if let Some(JsonValue::Array(features)) = map.get("\"features\"") {
                    println!("Feature count: {}", features.len());
                    for (i, feature) in features.iter().enumerate() {
                        if let JsonValue::String(f) = feature {
                            println!("  {i}. {f}");
                        }
                    }
                }
                if let Some(JsonValue::Object(meta)) = map.get("\"metadata\"") {
                    if let Some(JsonValue::String(escaped)) = meta.get("\"escaped_text\"") {
                        println!("Escaped text: {escaped}");
                    }
                }
            }
        }
        Err(err) => {
            eprintln!("Failed to parse JSON: {err}");
        }
    }

    println!("\n=== Error Handling Example (Trailing Comma) ===");
    let invalid_json = r#"{"name": "test",}"#;
    match json::Parser::lex_parse(invalid_json) {
        Ok(_) => println!("Unexpectedly succeeded!"),
        Err(err) => {
            println!("Correctly rejected invalid JSON:\n{err}");
        }
    }
}
