use super::ast::Member;
use semasia::*;
use std::collections::BTreeMap;

#[grammar]
#[logos(skip r"[ \t\n\r]+")]
pub mod json {
    use super::*;

    #[non_terminal]
    pub type JsonBool = bool;

    #[start_symbol]
    #[non_terminal]
    #[auto_productions]
    #[derive(Debug, Clone, PartialEq)]
    pub enum JsonValue {
        Null(#[hide] NullToken),
        Bool(JsonBool),
        Number(JsonNumber),
        String(JsonString),
        Array(JsonArray),
        Object(JsonObject),
    }

    #[non_terminal]
    pub type JsonObject = BTreeMap<String, JsonValue>;

    #[non_terminal]
    pub type JsonArray = Vec<JsonValue>;

    #[non_terminal]
    pub type JsonMember = Member;

    #[token("null")]
    pub struct NullToken;

    #[token("true")]
    pub struct TrueToken;

    #[token("false")]
    pub struct FalseToken;

    #[token("{")]
    pub struct OpenBrace;

    #[token("}")]
    pub struct CloseBrace;

    #[token("[")]
    pub struct OpenBracket;

    #[token("]")]
    pub struct CloseBracket;

    #[token(":")]
    pub struct Colon;

    #[token(",")]
    pub struct Comma;

    #[regex(r#""([^"\\]|\\.)*""#, to_string)]
    pub type JsonString = String;

    #[regex(r"-?(?:0|[1-9]\d*)(?:\.\d+)?(?:[eE][+-]?\d+)?", parse)]
    pub type JsonNumber = f64;

    production!(JsonBoolTrue: JsonBool -> TrueToken, |_| true);
    production!(JsonBoolFalse: JsonBool -> FalseToken, |_| false);

    production!(
        MemberDef: JsonMember -> (JsonString, Colon, JsonValue),
        |(key, _, value)| Member { key, value }
    );

    ebnf!(
        ObjectDef: JsonObject -> (OpenBrace, #[separator(Comma)] Vec<JsonMember>, CloseBrace),
        |(_, members, _)| {
            let mut map = BTreeMap::new();
            for member in members {
                map.insert(member.key, member.value);
            }
            map
        }
    );

    ebnf!(
        ArrayDef: JsonArray -> (OpenBracket, #[separator(Comma)] Vec<JsonValue>, CloseBracket),
        |(_, elements, _)| elements
    );
}

pub use json::JsonValue;
