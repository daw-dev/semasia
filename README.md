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
    assert_eq!(res, 22);
}
```

