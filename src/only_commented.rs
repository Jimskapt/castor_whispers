// TODO : use string slices instead of String ?
// TODO : detect & do not append markdownlint comment blocks (`markdownlint-disable`)
// TODO : detect & append uncommented code blocks (```) surrounded with (\n),
//        they should be commented blocks, but are re-used in uncommented text.
pub fn only_commented(input_content: &str) -> String {
	let lines: Vec<&str> = input_content.trim().split('\n').collect();

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
									new_content += &line
										[(start_pos + start_comment.len())..close_pos]
										.trim()
										.replace("-- >", "-->");
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
								new_content += &line[(start_pos + start_comment.len())..]
									.trim()
									.replace("-- >", "-->");
								new_content += "\n";

								status = ParseStatus::Comment;
							}
						}
						ParseStatus::Comment => {
							new_content += &line.replace("-- >", "-->");
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
								new_content += &line[..close_pos].replace("-- >", "-->");

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
								new_content += &line.replace("-- >", "-->");
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
