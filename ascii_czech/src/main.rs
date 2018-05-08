#![feature(io)]
use std::io::{Read, stdin};
use std::env::args;
use std::fs::File;

fn translate(c: char) {
	match c {
		'ř' => print!("\\o`r\\[ah]`"),
		'š' => print!("\\[vs]"),
		'č' => print!("\\o`c\\[ah]`"),
		'ě' => print!("\\o`e\\[ah]`"),
		'ž' => print!("\\[vz]"),
		'ň' => print!("\\o`n\\[ah]`"),
		'á' => print!("\\o`a\\(aa`"),
		'é' => print!("\\o`e\\(aa`"),
		'í' => print!("\\['i]"),
		'ó' => print!("\\o`o\\(aa`"),
		'ú' => print!("\\o`u\\(aa`"),
		'ý' => print!("\\o`y\\(aa`"),
		'ů' => print!("u\\*o"),
		'ť' => print!("\\o`t\\(aa`"),
		'ď' => print!("\\o`d\\(aa`"),
		'Š' => print!("\\[vS]"),
		'Ž' => print!("\\[vZ]"),
		'É' => print!("\\['E]"),
		'Á' => print!("\\['A]"),
		'Í' => print!("\\['I]"),
		'Ý' => print!("\\['Y]"),
		'Ú' => print!("\\['U]"),
		'Ů' => print!("u\\*o"),
		'Ň' => print!("\\zN\\v`-.2v`\\h`.2v`\\[ah]\\v`.2v`\\|"),
		'Č' => print!("\\zC\\v`-.2v`\\h`.15v`\\[ah]\\v`.2v`\\|"),
		'Ř' => print!("\\zR\\v`-.2v`\\h`.15v`\\[ah]\\v`.2v`\\|"),
		'Ě' => print!("\\zE\\v`-.2v`\\h`.15v`\\[ah]\\v`.2v`\\|"),
		'Ď' => print!("\\zD\\v`-.2v`\\h`.15v`\\[ah]\\v`.2v`\\|"),
		'Ť' => print!("\\zT\\v`-.2v`\\h`.15v`\\[ah]\\v`.2v`\\|"),
		x   => print!("{}", x),
	}
}

fn main() {
	for arg in args().skip(1) {
		match arg.as_ref() {
			"-" => stdin().chars().map(|x| x.unwrap()).for_each(|c| translate(c)),
			x => if let Ok(f) = File::open(x) {
				f.chars().map(|x| x.unwrap()).for_each(|c| translate(c))
			}
			else { eprintln!("failed to open file: {}", x) }
		}
	}
}

