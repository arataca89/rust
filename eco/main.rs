// package eco

use clap::{App, Arg};

fn main() {
    let matches = App::new("eco")
		.version("0.1.0")
		.author("arataca89 <arataca89@gmail.com>")
		.about("Clone do comando echo")
		.arg(
			Arg::with_name("text")
				.value_name("TEXT")
				.help("Input text")
				.required(true)
				.min_values(1),
		)
		.arg(
			Arg::with_name("omit_newline")
				.help("Do not print newline")
				.takes_value(false)
				.short("n"),
		)    
		.get_matches();
	let text = matches.values_of_lossy("text").unwrap();
	let omit_newline = matches.is_present("omit_newline");
	print!("{}{}", text.join(" "), if omit_newline {""} else {"\n"});	
}
