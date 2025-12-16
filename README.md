# Static SDD

A compile-time in-place parser generator written in rust.

## Usage

To specify this crate as a dependency on your project simply run `cargo add --git https://github.com/daw-dev/static_sdd` or add the follwing to your `Cargo.toml`:

```toml
[dependency]
static_sdd = { git = "https://github.com/daw-dev/static_sdd" }
```

Then, anywhere in your project:

```rust
use static_sdd::*;

#[grammar]
mod addition {
    use super::*;

    #[non_terminal]
    #[start_symbol]
    pub type E = usize;

    #[token = r"\d+"]
    pub type Num = usize;

    #[token = "+"]
    pub struct Plus;

    production!(P0, E -> (E, Plus, Num), |(e, _, num)| e + num);

    production!(P1, E -> Num);
}

fn main() {
    let res = addition::parse("10+3+9");
    assert_eq!(res, Ok(22));
}
```

## Tool Comparison

What follows is a small comparison with tools that are in different ways similar this one:

### Rust Parser Generators

The following are rust parser generators - same category as this tool - so they all have some features in common:

- bottom up parsing
- semantic actions are called during parsing

| Feature                | This Tool                                                  | LALRPOP                              | grmtools (lrpar)                          | Pomelo                           |
|------------------------|------------------------------------------------------------|--------------------------------------|-------------------------------------------|----------------------------------|
| Philosophy             | Use rust type system and module system to define a grammar | Rust version of bison                | Bison-compatible parser generator in rust | Rust version of lemon            |
| Algorithm              | LALR(1)                                                    | LALR(1)/LR(1)                        | LR(1)/GLR                                 | LALR(1) (lemon)                  |
| Execution Time         | Compile time (proc macro attribute)                        | Compile Time (build.rs)              | Compile Time (build.rs)                   | Compile Time (proc macro)        |
| Lexing                 | Internal (custom implementation or logos.rs)               | Internal (regex) or External         | External (lrlex)                          | External (expects Token enum)    |
| Synthesized Attributes | Yes (return types)                                         | Yes (return types)                   | Yes                                       | Yes (types)                      |
| Inherited Attributes   | Yes (helper types and compiler context)                    | No                                   | No                                        | No (%extra_args)                 |
| Zero-Copy              | Yes                                                        | Limited                              | Limited                                   | No                               |
| Error recovery         | Expressive errors and suggestions                          | !token / Recovery                    | Advanced (CPCT+)                          | No (panic!)                      |
| Grammar Definition     | Attributes inside a normal rust module, production! macro  | .lalrpop file with custom syntax     | .y file with Yacc syntax (mostly)         | pomelo! macro with custom syntax |
| IDE Support            | Works with rust-analyzer                                   | Custom LSP                           | Yacc LSP                                  | Very limited                     |

### Foreign Parser Generators

The following are also parser generators, but they have a different target language, the features will be similar to the ones above

| Feature                | Bison                            | ANTLR4                                | Menhir                               |
|------------------------|----------------------------------|---------------------------------------|--------------------------------------|
| Target Language        | C/C++                            | C++/C#/Java/js/PHP/Python/Swift/TS/GO | OCaml                                |
| Algorithm              | LALR(1)/GLR                      | Adaptive LL(*)                        | LR(1)                                |
| Execution Time         | Ahead of time (generates C code) | Ahead of time (generates code)        | Ahead of time (generates OCaml code) |
| Lexing                 | External (flex)                  | Internal                              | External                             |
| Synthesized Attributes | Yes ($$)                         | Yes                                   | Yes                                  |
| Inherited Attributes   | Yes (through mid-rule actions)   | Yes (discouraged)                     | Not really                           |
| Zero-Copy              | No                               | No                                    | No                                   |

### Alternative Approaches (Non-LALR)

These tools use different parsing philosophies compared to bottom-up LR/LALR generators. They are often preferred for binary formats or when a separate grammar file is undesirable.

| Feature | Parser Combinators (nom, chumsky) | PEG Generators (pest) | Tree-sitter |
| :--- | :--- | :--- | :--- |
| Category | Parser Combinators | PEG Parser Generator | Incremental GLR Parser |
| Philosophy | Grammar is defined as executable Rust functions | Grammar defined in external `.pest` file | Error-resilient parsing designed for IDEs |
| Algorithm | Recursive Descent (LL) | Packrat / PEG (Top-down) | GLR (Generalized LR) |
| Execution | Runtime (Function composition) | Compile time (Generates recursive descent) | Runtime (C runtime with Rust bindings) |
| Lexing | Integrated (Byte/Char stream) | Integrated (Regex-like) | Integrated |
| Zero-Copy | Yes (First-class citizen) | Yes | No (creates concrete syntax tree) |
| Ambiguity | Manual resolution (`alt` / `try`) | Prioritized Choice (`/` operator) | Handles ambiguity automatically (GLR) |
| Best For | Binary formats, network protocols, small DSLs | Config files, simple markup languages | Syntax highlighting, code analysis, fuzzy parsing |

