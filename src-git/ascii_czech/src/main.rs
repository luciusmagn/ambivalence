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

fn untranslate(line: &str) {
	println!("{}", line
		.replace("\\o`r\\[ah]`", "ř")
		.replace("\\[vs]", "š")
		.replace("\\o`c\\[ah]`", "č")
		.replace("\\o`e\\[ah]`", "ě")
		.replace("\\[vz]", "ž")
		.replace("\\o`n\\[ah]`", "ň")
		.replace("\\o`a\\(aa`", "á")
		.replace("\\o`e\\(aa`", "é")
		.replace("\\['i]", "í")
		.replace("\\o`o\\(aa`", "ó")
		.replace("\\o`u\\(aa`", "ú")
		.replace("\\o`y\\(aa`", "ý")
		.replace("\\zu\\v`-.1v`\\h`.05v`\\[de]\\v`.1v`", "ů")
		.replace("\\zd\\(aa", "ď")
		.replace("\\zt\\(aa", "ť")
		.replace("\\[vS]", "Š")
		.replace("\\[vZ]", "Ž")
		.replace("\\['E]", "É")
		.replace("\\['A]", "Á")
		.replace("\\['I]", "Í")
		.replace("\\['Y]", "Ý")
		.replace("\\['U]", "Ú")
		.replace("\\['O]", "Ó")
		.replace("\\zC\\v`-.2v`\\h`.15v`\\[ah]\\v`.2v`\\|", "Č")
		.replace("\\zŘ\\v`-.2v`\\h`.15v`\\[ah]\\v`.2v`\\|", "Ř")
		.replace("\\zE\\v`-.2v`\\h`.15v`\\[ah]\\v`.2v`\\|", "Ě")
		.replace("\\zD\\v`-.2v`\\h`.15v`\\[ah]\\v`.2v`\\|", "Ď")
		.replace("\\zT\\v`-.2v`\\h`.15v`\\[ah]\\v`.2v`\\|", "Ť")
	);
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

