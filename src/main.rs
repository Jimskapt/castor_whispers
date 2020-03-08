fn main() {
	let app = clap::App::new("Castor Whispers")
				.version(env!("CARGO_PKG_VERSION"))
				.about("An utility to work with markdown text files")
				.subcommand(
					clap::SubCommand::with_name("comment")
						.about("Comment all paragraphs")
						.arg(
							clap::Arg::with_name("input")
								.help("The input file which contains all paragraphs")
								.required(true)
						)
				)
				/*
				.subcommand(
					clap::SubCommand::with_name("uncomment")
						.about("Get only uncommented paragraphs")
						.arg(
							clap::Arg::with_name("input")
								.help("The input file which contains all paragraphs")
								.required(true)
						)
				)
				*/
				/*
				.subcommand(
					clap::SubCommand::with_name("only-comment")
						.about("Get only commented paragraphs")
						.arg(
							clap::Arg::with_name("input")
								.help("The input file which contains all paragraphs")
								.required(true)
						)
				)
				*/;
	let mut help_text = Vec::new();
	app.write_help(&mut help_text).unwrap();

	let matches = app.get_matches();

	let search_code_delemiter = regex::Regex::new(r"(`+``+)").unwrap();

	if let Some(subcommand) = matches.subcommand_matches("comment") {
		let input_path = subcommand
			.value_of("input")
			.expect("Please supply input file path");
		let input_content =
			std::fs::read_to_string(input_path).expect("error while reading input file");

		let lines: Vec<&str> = input_content.split("\n").collect();

		let mut new_content = String::new();

		let mut previous: Option<&str> = None;
		let mut status: ParseStatus = ParseStatus::None;
		for line in lines {
			if status == ParseStatus::None {
				match previous {
					Some(text) => {
						if text.trim() == "" {
							new_content.push_str("<!--\n");
						}
					}
					None => {
						new_content.push_str("<!--\n");
					}
				}

				if line.trim() == "" {
					new_content.push_str("-->\n");
				}
			}

			// TODO : find case when openning ``` and closing ``` on same line.
			match search_code_delemiter.captures(line) {
				Some(found) => {
					let delimiter = found[1].to_owned();

					match &status {
						ParseStatus::Code(current_delimiter) => {
							if delimiter == *current_delimiter {
								status = ParseStatus::None;
							}
						}
						ParseStatus::None => {
							status = ParseStatus::Code(delimiter.clone());
						}
					}
				}
				None => {}
			}

			new_content.push_str(line.replace("-->", "-- >").as_str());
			new_content.push_str("\n");

			previous = Some(line.trim());
		}

		println!("{}", new_content);
	} else {
		println!("{}", std::str::from_utf8(&help_text).unwrap());
	}
}

#[derive(PartialEq)]
enum ParseStatus {
	None,
	Code(String),
}
