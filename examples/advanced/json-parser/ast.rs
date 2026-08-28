use super::grammar::JsonValue;
use ptree::{TreeBuilder, item::StringItem};
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub struct Member {
    pub key: String,
    pub value: JsonValue,
}

impl JsonValue {
    pub fn build_tree(&self) -> StringItem {
        let mut builder = TreeBuilder::new(String::from("JSON:"));
        self.build_tree_recursive(&mut builder, None);
        builder.build()
    }

    fn build_tree_recursive(&self, builder: &mut TreeBuilder, key: Option<&str>) {
        let prefix = match key {
            Some(k) => format!("{k}: "),
            None => String::new(),
        };

        match self {
            JsonValue::Null => {
                builder.add_empty_child(format!("{prefix}null"));
            }
            JsonValue::Bool(val) => {
                builder.add_empty_child(format!("{prefix}{val}"));
            }
            JsonValue::Number(n) => {
                builder.add_empty_child(format!("{prefix}{n}"));
            }
            JsonValue::String(s) => {
                builder.add_empty_child(format!("{prefix}{s}"));
            }
            JsonValue::Array(items) => {
                builder.begin_child(format!("{prefix}[Array, {} items]", items.len()));
                for (idx, item) in items.iter().enumerate() {
                    item.build_tree_recursive(builder, Some(&format!("[{idx}]")));
                }
                builder.end_child();
            }
            JsonValue::Object(map) => {
                builder.begin_child(format!("{prefix}{{Object, {} entries}}", map.len()));
                for (k, val) in map {
                    val.build_tree_recursive(builder, Some(k.as_str()));
                }
                builder.end_child();
            }
        }
    }
}

impl fmt::Display for JsonValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fn fmt_indent(val: &JsonValue, f: &mut fmt::Formatter<'_>, indent: usize) -> fmt::Result {
            let spaces = "  ".repeat(indent);
            let inner_spaces = "  ".repeat(indent + 1);

            match val {
                JsonValue::Null => write!(f, "null"),
                JsonValue::Bool(val) => write!(f, "{val}"),
                JsonValue::Number(n) => write!(f, "{n}"),
                JsonValue::String(s) => write!(f, "{s}"),
                JsonValue::Array(items) => {
                    if items.is_empty() {
                        write!(f, "[]")
                    } else {
                        writeln!(f, "[")?;
                        for (i, item) in items.iter().enumerate() {
                            write!(f, "{inner_spaces}")?;
                            fmt_indent(item, f, indent + 1)?;
                            if i + 1 < items.len() {
                                writeln!(f, ",")?;
                            } else {
                                writeln!(f)?;
                            }
                        }
                        write!(f, "{spaces}]")
                    }
                }
                JsonValue::Object(map) => {
                    if map.is_empty() {
                        write!(f, "{{}}")
                    } else {
                        writeln!(f, "{{")?;
                        for (i, (key, value)) in map.iter().enumerate() {
                            write!(f, "{inner_spaces}{key}: ")?;
                            fmt_indent(value, f, indent + 1)?;
                            if i + 1 < map.len() {
                                writeln!(f, ",")?;
                            } else {
                                writeln!(f)?;
                            }
                        }
                        write!(f, "{spaces}}}")
                    }
                }
            }
        }

        fmt_indent(self, f, 0)
    }
}
