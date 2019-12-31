use std::env::args;
use std::fs::metadata;
use std::process::{Command, exit, Stdio};

fn main() -> Result<(), ()> {
    let name = args().skip(1).next().ok_or(())?;
    loop {
    	let pdf_metadata = metadata((&name).to_string() + ".pdf").map_err(|_| eprintln!("couldn't access .pdf metadata"))?;
    	let me_metadata = metadata((&name).to_string() + ".me").map_err(|_| eprintln!("couldn't access .me metadata"))?;

    	let pdf_date = pdf_metadata
    		.modified()
    		.unwrap_or_else(|_| { eprintln!("couldn't get date"); exit(-1) });
    	let me_date = me_metadata
    		.modified()
    		.unwrap_or_else(|_| { eprintln!("couldn't get date"); exit(-1) });

    	if pdf_date < me_date
    	{
    		if !Command::new("./fs/bin/update.sh")
			.arg(&name)
    			.stdout(Stdio::inherit())
    			.stderr(Stdio::inherit())
    			.spawn()
    			.expect("couldn't run script")
    			.wait()
    			.expect("failed to wait")
    			.success()
    		{
    			eprintln!("script errored")
    		}
    	}
	else {
		std::thread::sleep(std::time::Duration::from_millis(800))
	}
    }
}
