use std::io::{Read, stdin};
use std::env::args;
use std::fs::File;
use std::str;

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
		'ů' => print!("\\zu\\v`-.1v`\\h`.05v`\\[de]\\v`.1v`"),
		'ď' => print!("\\zd\\(aa"),
		'ť' => print!("\\zt\\(aa"),
		'Š' => print!("\\[vS]"),
		'Ž' => print!("\\[vZ]"),
		'É' => print!("\\['E]"),
		'Á' => print!("\\['A]"),
		'Í' => print!("\\['I]"),
		'Ý' => print!("\\['Y]"),
		'Ú' => print!("\\['U]"),
		'Ó' => print!("\\['O]"),
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
			"-" => str::from_utf8(&stdin().bytes().map(|x| x.unwrap()).collect::<Vec<u8>>()).unwrap().chars().for_each(|c| translate(c)),
			x => if let Ok(f) = File::open(x) {
				str::from_utf8(&f.bytes().map(|x| x.unwrap()).collect::<Vec<u8>>()).unwrap().chars().for_each(|c| translate(c))
			}
			else { eprintln!("failed to open file: {}", x) }
		}
	}
}

