extern crate scheme2c;

fn main() {
	let sexp = r#"("hello" 2)"#;
	let parsed = scheme2c::parser::SexpParser::new().parse(&sexp).unwrap();
    println!("{:#?}", parsed);
}
