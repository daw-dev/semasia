mod functions;

fn main() {
    let res = functions::Parser::lex_parse("x2+2");
    match res {
        Ok(res) => println!("{res:?}"),
        Err(err) => println!("{err}"),
    }
}
