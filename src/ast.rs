#[derive(Debug, PartialEq, Clone)]
pub enum Sexp {
	Str(String),
	Symbol(String),
	Int(i32),
	Float(f32),
	Bool(bool),
	List {
		atoms: Vec<Box<Sexp>>,
		proper: bool,
	},
	Nil,
}