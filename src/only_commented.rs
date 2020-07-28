// TODO : use string slices instead of String ?
// TODO : detect & do not append markdownlint comment blocks (`markdownlint-disable`)
// TODO : detect & append uncommented code blocks (```) surrounded with (\n),
//        they should be commented blocks, but are re-used in uncommented text.
pub fn only_commented(input_content: &str, settings: &crate::settings::Settings) -> String {
	let lines: Vec<&str> = input_content.trim().split('\n').collect();

	if let Some(comments) = &settings.replacements.only_commented {
		for (search, _) in comments {
			if let Err(e) = regex::Regex::new(&search) {
				eprintln!("Error while parsing regex {} : {}", search, e);
			}
		}
	}

	let mut new_content = String::new();

	let mut status: ParseStatus = ParseStatus::None;
	let start_comment = "<!--";
	for line in lines {
		let start_search = line.find(start_comment);
		let close_search = line.find("-->");

		match start_search {
			Some(start_pos) => {
				match close_search {
					Some(close_pos) => {
						match status {
							ParseStatus::None => {
								if line[..start_pos].trim() == "" {
									new_content += "\n";

									let mut temp = line
										[(start_pos + start_comment.len())..close_pos]
										.trim()
										.to_string();

									if let Some(comments) = &settings.replacements.only_commented {
										for (search, replace) in comments {
											if let Ok(rgx) = regex::Regex::new(&search) {
												temp = rgx
													.replace_all(&temp, replace.as_str())
													.to_string();
											}
										}
									}

									temp = temp.replace("-- >", "-->");

									new_content += &temp;

									new_content += "\n";
								}

								// do nothing with status because we open and close a comment on same line
							}
							ParseStatus::Comment => {
								new_content += &line.replace("-- >", "-->");
							}
						}
					}
					None => match status {
						ParseStatus::None => {
							if line[..start_pos].trim() == "" {
								let mut temp =
									line[(start_pos + start_comment.len())..].trim().to_string();

								if let Some(comments) = &settings.replacements.only_commented {
									for (search, replace) in comments {
										if let Ok(rgx) = regex::Regex::new(&search) {
											temp = rgx
												.replace_all(&temp, replace.as_str())
												.to_string();
										}
									}
								}

								temp = temp.replace("-- >", "-->");

								new_content += &temp;
								new_content += "\n";

								status = ParseStatus::Comment;
							}
						}
						ParseStatus::Comment => {
							let mut temp = String::from(line);

							if let Some(comments) = &settings.replacements.only_commented {
								for (search, replace) in comments {
									if let Ok(rgx) = regex::Regex::new(&search) {
										temp = rgx.replace_all(&temp, replace.as_str()).to_string();
									}
								}
							}

							temp = temp.replace("-- >", "-->");

							new_content += &temp;
							new_content += "\n";
						}
					},
				}
			}
			None => {
				match close_search {
					Some(close_pos) => {
						match status {
							ParseStatus::Comment => {
								let mut temp = String::from(&line[..close_pos]);

								if let Some(comments) = &settings.replacements.only_commented {
									for (search, replace) in comments {
										if let Ok(rgx) = regex::Regex::new(&search) {
											temp = rgx
												.replace_all(&temp, replace.as_str())
												.to_string();
										}
									}
								}

								temp = temp.replace("-- >", "-->");

								new_content += &temp;

								status = ParseStatus::None;
							}
							ParseStatus::None => {
								// do nothing voluntary
							}
						}
					}
					None => {
						match status {
							ParseStatus::Comment => {
								let mut temp = String::from(line);

								if let Some(comments) = &settings.replacements.only_commented {
									for (search, replace) in comments {
										if let Ok(rgx) = regex::Regex::new(&search) {
											temp = rgx
												.replace_all(&temp, replace.as_str())
												.to_string();
										}
									}
								}

								temp = temp.replace("-- >", "-->");

								new_content += &temp;
								new_content += "\n";
							}
							ParseStatus::None => {
								// do nothing voluntary
							}
						}
					}
				}
			}
		}
	}

	return new_content.trim().into();
}

#[derive(PartialEq)]
enum ParseStatus {
	None,
	Comment,
}
