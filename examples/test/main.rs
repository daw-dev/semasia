#[semasia::grammar]
mod calculator {
    use semasia::production;

    #[start_symbol]
    #[non_terminal]
    pub type Expr = f32;

    #[regex(r"\d+", parse)]
    #[regex(r"\d*\.\d+", parse)]
    pub type Num = f32;

    production!(Number: Expr -> Num);
}

use calculator::Parser;

fn main() {
    let res = Parser::lex_parse("5128");
    match res {
        Ok(res) => println!("{res}"),
        Err(err) => eprintln!("{err}"),
    }
}
