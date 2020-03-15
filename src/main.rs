mod comment;
mod only_commented;
mod only_uncommented;

mod tests;

fn main() {
	// TODO : refactor inputs in order to allow use of cli streams ?
	let app = clap::App::new("castor_whispers")
		.version(env!("CARGO_PKG_VERSION"))
		.about("An utility to work with markdown text files")
		.subcommand(
			clap::SubCommand::with_name("comment")
				.about("Comment all paragraphs")
				.arg(
					clap::Arg::with_name("input")
						.help("The input file which contains all paragraphs")
						.required(true),
				),
		)
		/*
		.subcommand(
			clap::SubCommand::with_name("only-uncommented")
				.about("Get only uncommented paragraphs")
				.arg(
					clap::Arg::with_name("input")
						.help("The input file which contains all paragraphs")
						.required(true),
				),
		)
		*/
		.subcommand(
			clap::SubCommand::with_name("only-commented")
				.about("Get only commented paragraphs")
				.arg(
					clap::Arg::with_name("input")
						.help("The input file which contains all paragraphs")
						.required(true),
				),
		);
	let mut help_text = Vec::new();
	app.write_help(&mut help_text).unwrap();

	let matches = app.get_matches();

	if let Some(subcommand) = matches.subcommand_matches("comment") {
		let input_path = subcommand
			.value_of("input")
			.expect("Please supply input file path");
		let input_content = std::fs::read_to_string(input_path)
			.unwrap_or_else(|_| panic!("error while reading input file {}", input_path));

		let new_content = comment::comment(&input_content);

		println!("{}", new_content);
	} else if let Some(subcommand) = matches.subcommand_matches("only-commented") {
		let input_path = subcommand
			.value_of("input")
			.expect("Please supply input file path");
		let input_content = std::fs::read_to_string(input_path)
			.unwrap_or_else(|_| panic!("error while reading input file {}", input_path));

		let new_content = only_commented::only_commented(&input_content);

		println!("{}", new_content);
	}
	/*else if let Some(subcommand) = matches.subcommand_matches("only-uncommented") {
		let input_path = subcommand
			.value_of("input")
			.expect("Please supply input file path");
		let input_content = std::fs::read_to_string(input_path)
			.unwrap_or_else(|_| panic!("error while reading input file {}", input_path));

		let new_content = only_uncommented::only_uncommented(&input_content);

		println!("{}", new_content);
	}*/
	else {
		println!("{}", std::str::from_utf8(&help_text).unwrap());
	}
}
