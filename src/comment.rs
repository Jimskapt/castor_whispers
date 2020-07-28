// TODO : refactor in order to use string slices ?
pub fn comment(input_content: &str, settings: &crate::settings::Settings) -> String {
	let search_code_delemiter = regex::Regex::new(r"(`+``+)").unwrap();

	let lines: Vec<&str> = input_content.trim().split('\n').collect();

	if let Some(comments) = &settings.replacements.comment {
		for (search, _) in comments {
			if let Err(e) = regex::Regex::new(&search) {
				eprintln!("Error while parsing regex {} : {}", search, e);
			}
		}
	}

	let mut new_content = String::new();

	let mut previous: Option<&str> = None;
	let mut status: ParseStatus = ParseStatus::None;
	let mut comment_on = false;
	for line in lines {
		if status == ParseStatus::None {
			match previous {
				Some(text) => {
					if text.trim() == "" {
						new_content.push_str("<!--\n");
						comment_on = true;
					}
				}
				None => {
					new_content.push_str("<!--\n");
					comment_on = true;
				}
			}

			if line.trim() == "" {
				new_content.push_str("-->\n");
				comment_on = false;
			}
		}

		// TODO : find case when openning ``` and closing ``` on same line.
		// TODO : adapt on multi-level code, like ````, `````, ``````, ...
		if let Some(found) = search_code_delemiter.captures(line) {
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

		let mut temp = String::from(line);

		if let Some(comments) = &settings.replacements.comment {
			for (search, replace) in comments {
				if let Ok(rgx) = regex::Regex::new(&search) {
					temp = rgx.replace_all(&temp, replace.as_str()).to_string();
				}
			}
		}

		temp = temp.replace("-->", "-- >");

		new_content.push_str(temp.as_str());
		new_content.push_str("\n");

		previous = Some(line.trim());
	}

	if comment_on {
		new_content.push_str("-->");
		comment_on = false; // this is just a precaution in case the code that follows this line evolves.
	}

	return new_content;
}

#[derive(PartialEq)]
enum ParseStatus {
	None,
	Code(String),
}
