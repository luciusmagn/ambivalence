#![feature(io)]
use std::io::{Read, stdin};
use std::env::args;
use std::fs::File;

fn translate(c: char) {
	match c {
		'ř' => print!("\\o`r\\(aa\\(ga`"),
		'š' => print!("\\o`s\\(aa\\(ga`"),
		'č' => print!("\\o`c\\(aa\\(ga`"),
		'ě' => print!("\\o`e\\(aa\\(ga`"),
		'ž' => print!("\\o`z\\(aa\\(ga`"),
		'ň' => print!("\\o`n\\(aa\\(ga`"),
		'ť' => print!("\\o`t\\(aa`"),
		'á' => print!("\\o`a\\(aa`"),
		'é' => print!("\\o`e\\(aa`"),
		'í' => print!("\\o`i\\(aa`"),
		'ó' => print!("\\o`o\\(aa`"),
		'ú' => print!("\\o`u\\(aa`"),
		'ý' => print!("\\o`y\\(aa`"),
		'ů' => print!("\\zu\\u\\h`0.15`\\s-6o\\s+6\\d "), //wtf
		x   => print!("{}", x),
	}
}

fn main() {
	for arg in args().skip(1) {
		let mut tmp: [u8; 4] = [0; 4];
		match arg.as_ref() {
			"-" => stdin().chars().map(|x| x.unwrap()).for_each(|c| translate(c)),
			x => if let Ok(f) = File::open(x) {
				f.chars().map(|x| x.unwrap()).for_each(|c| translate(c))
			}
			else { eprintln!("failed to open file: {}", x) }
		}
	}
}

