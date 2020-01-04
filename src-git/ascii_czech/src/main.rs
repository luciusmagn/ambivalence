extern crate rreplace;

use std::collections::HashMap;
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

fn untranslate(c: &str) {
	println!("{}", rreplace::run(c, &{
		let mut h = HashMap::new();
		h.insert("\\o`r\\[ah]`", "ř");
		h.insert("\\[vs]", "š");
		h.insert("\\o`c\\[ah]`", "č");
		h.insert("\\o`e\\[ah]`", "ě");
		h.insert("\\[vz]", "ž");
		h.insert("\\o`n\\[ah]`", "ň");
		h.insert("\\o`a\\(aa`", "á");
		h.insert("\\o`e\\(aa`", "é");
		h.insert("\\['i]", "í");
		h.insert("\\o`o\\(aa`", "ó");
		h.insert("\\o`u\\(aa`", "ú");
		h.insert("\\o`y\\(aa`", "ý");
		h.insert("\\zu\\v`-.1v`\\h`.05v`\\[de]\\v`.1v`", "ů");
		h.insert("\\zd\\(aa", "ď");
		h.insert("\\zt\\(aa", "ť");
		h.insert("\\[vS]", "Š");
		h.insert("\\[vZ]", "Ž");
		h.insert("\\['E]", "É");
		h.insert("\\['A]", "Á");
		h.insert("\\['I]", "Í");
		h.insert("\\['Y]", "Ý");
		h.insert("\\['U]", "Ú");
		h.insert("\\['O]", "Ó");
		h.insert("\\zC\\v`-.2v`\\h`.15v`\\[ah]\\v`.2v`\\|", "Č");
		h.insert("\\zŘ\\v`-.2v`\\h`.15v`\\[ah]\\v`.2v`\\|", "Ř");
		h.insert("\\zE\\v`-.2v`\\h`.15v`\\[ah]\\v`.2v`\\|", "Ě");
		h.insert("\\zD\\v`-.2v`\\h`.15v`\\[ah]\\v`.2v`\\|", "Ď");
		h.insert("\\zT\\v`-.2v`\\h`.15v`\\[ah]\\v`.2v`\\|", "Ť");
		h
	}));
}

fn main() {
	match args().skip(1).next().unwrap_or_default().as_ref() {
		"fuck" => for arg in args().skip(2) {
			match arg.as_ref() {
				"-" => String::from_utf8_lossy(&stdin().bytes().map(|x| x.unwrap()).collect::<Vec<u8>>()).chars().for_each(|c| translate(c)),
				x => if let Ok(f) = File::open(x) {
					String::from_utf8_lossy(&f.bytes().map(|x| x.unwrap()).collect::<Vec<u8>>()).chars().for_each(|c| translate(c))
				}
				else { eprintln!("failed to open file: {}", x) }
			}
		},
		"unfuck" => for arg in args().skip(2) {
			match arg.as_ref() {
				"-" => String::from_utf8_lossy(&stdin().bytes().map(|x| x.unwrap()).collect::<Vec<u8>>()).lines().for_each(|c| untranslate(c)),
				x => if let Ok(f) = File::open(x) {
					String::from_utf8_lossy(&f.bytes().map(|x| x.unwrap()).collect::<Vec<u8>>()).lines().for_each(|c| untranslate(c))
				}
				else { eprintln!("failed to open file: {}", x) }
			}
		},
		_ => eprintln!("invalid subcommand"),
	}	
}

