use std::fs::metadata;
use std::process::{Command, exit, Stdio};

fn main() {
    loop {
    	let pdf_metadata = match metadata("ambivalence.pdf") {
    		Ok(m) => m,
    		Err(_) => { eprintln!("couldn't access .pdf metadata"); return }
    	};
    	let me_metadata = match metadata("ambivalence.me") {
    		Ok(m) => m,
    		Err(_) => { eprintln!("couldn't access .me metadata"); return }
    	};

    	let pdf_date = pdf_metadata
    		.modified()
    		.unwrap_or_else(|_| { eprintln!("couldn't get date"); exit(-1) });
    	let me_date = me_metadata
    		.modified()
    		.unwrap_or_else(|_| { eprintln!("couldn't get date"); exit(-1) });

    	if pdf_date < me_date
    	{
    		if !Command::new("./update.sh")
    			.stdout(Stdio::inherit())
    			.output()
    			.expect("couldn't run script")
    			.status
    			.success()
    		{
    			eprintln!("script errored")
    		}
    	}
    }
}
