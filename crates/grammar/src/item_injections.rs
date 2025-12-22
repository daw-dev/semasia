use dyn_grammar::{non_terminal::NonTerminal, token::Token, Grammar};
use proc_macro2::Span;
use syn::{Ident, Item, parse_quote};

pub fn inject_items(items: &mut Vec<Item>, grammar: Grammar, compiler_ctx: Option<Ident>) {
    items.push(compiler_context(compiler_ctx));
    items.push(token_enum(grammar.tokens()));
    items.push(non_terminal_enum(grammar.non_terminals()));
    items.push(symbol_enum());
    // items.push(parse_one_fn());
    // items.push(lex_fn());
    // items.push(parse_fn(todo!()));
    // items.push(parse_str_fn(todo!()));
}

fn compiler_context(compiler_ctx: Option<Ident>) -> Item {
    compiler_ctx
        .map(|ctx| {
            parse_quote! {
                type __CompilerContext = #ctx;
            }
        })
        .unwrap_or(parse_quote! {
            type __CompilerContext = ();
        })
}

fn token_enum(tokens: &Vec<Token>) -> Item {
    let tokens = tokens.iter().map(|token| Ident::new(token.name(), Span::call_site()));
    parse_quote! {
        pub enum Token {
            #(#tokens (#tokens),)*
        }
    }
}

fn non_terminal_enum(non_terminals: &Vec<NonTerminal>) -> Item {
    let non_terminals = non_terminals.iter().map(|non_terminal| Ident::new(non_terminal.name(), Span::call_site()));
    parse_quote! {
        pub enum NonTerminal {
            #(#non_terminals (#non_terminals),)*
        }
    }
}

fn symbol_enum() -> Item {
    parse_quote! {
        pub enum Symbol {
            Token(Token),
            NonTerminal(NonTerminal),
        }
    }
}

fn parse_one_fn() -> Item {
    parse_quote! {
        pub fn parse_one(ctx: &mut __CompilerContext, stack: &mut Stack, curr: Token) {

        }
    }
}

fn parse_fn(start_symbol: Ident) -> Item {
    parse_quote! {
        pub fn parse(ctx: __CompilerContext, token_stream: impl IntoIterator<Token>) -> #start_symbol {
            todo!()
        }
    }
}

fn lex_fn() -> Item {
    parse_quote! {
        pub fn lex(word: impl Into<String>) -> Lex {
            todo!()
        }
    }
}

fn parse_str_fn(start_symbol: Ident) -> Item {
    parse_quote! {
        pub fn parse_str(ctx: __CompilerContext, word: impl Into<String>) -> #start_symbol {
            parse(ctx, lex(word))
        }
    }
}
