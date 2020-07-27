// TODO : use string slices instead of String ?
// TODO : detect & do not append markdownlint comment blocks (`markdownlint-disable`)
// TODO : detect & append uncommented code blocks (```) surrounded with (\n),
//        they should be commented blocks, but are re-used in uncommented text.
pub fn only_commented(input_content: &str) -> String {
	let lines: Vec<&str> = input_content.trim().split('\n').collect();

	let start_comment = "<!--";
	let search_code_delemiter = regex::Regex::new("(.*)(``+`)(.*)").unwrap(); // match only on one line, no \n possible !

	let mut comment_status: ParseCommentStatus = ParseCommentStatus::None;
	let mut code_status: ParseCodeStatus = ParseCodeStatus::None;

	let mut new_content = String::new();
	let mut comment_buffer = String::new();
	let mut code_buffer = String::new();

	let mut code_is_closed = false;

	let mut previous_line = String::new();
	for line in lines {
		let start_search = line.find(start_comment);
		let close_search = line.find("-->");

		match start_search {
			Some(start_pos) => {
				match close_search {
					Some(close_pos) => {
						match comment_status {
							ParseCommentStatus::None => {
								if line[..start_pos].trim() == "" {
									comment_buffer += "\n";
									comment_buffer += &line
										[(start_pos + start_comment.len())..close_pos]
										.trim()
										.replace("-- >", "-->");
									comment_buffer += "\n";
								}

								// do nothing with comment_status because we open and close a comment on same line
							}
							ParseCommentStatus::Comment => {
								comment_buffer += &line.replace("-- >", "-->");
							}
						}

						{
							match &code_status {
								ParseCodeStatus::Code(_) => {
									if code_is_closed {
										let trimmed = &comment_buffer.trim();
										let is_code_comment = &trimmed[..3] == "```"
											&& &trimmed[trimmed.len() - 3..] == "```";
										if !is_code_comment {
											new_content += &code_buffer;
											new_content += "\n";
										}
									}
								}
								ParseCodeStatus::None => {
									// voluntary do nothing
								}
							}

							code_status = ParseCodeStatus::None;
						}

						new_content += &comment_buffer;
						comment_buffer = String::new();
					}
					None => match comment_status {
						ParseCommentStatus::None => {
							if line[..start_pos].trim() == "" {
								comment_buffer += &line[(start_pos + start_comment.len())..]
									.trim()
									.replace("-- >", "-->");
								comment_buffer += "\n";

								comment_status = ParseCommentStatus::Comment;
							}
						}
						ParseCommentStatus::Comment => {
							comment_buffer += &line.replace("-- >", "-->");
							comment_buffer += "\n";
						}
					},
				}
			}
			None => {
				match close_search {
					Some(close_pos) => {
						match comment_status {
							ParseCommentStatus::Comment => {
								comment_buffer += &line[..close_pos].replace("-- >", "-->");

								comment_status = ParseCommentStatus::None;
							}
							ParseCommentStatus::None => {
								// do nothing voluntary
							}
						}

						{
							match &code_status {
								ParseCodeStatus::Code(_) => {
									if code_is_closed {
										let trimmed = &comment_buffer.trim();
										let is_code_comment = &trimmed[..3] == "```"
											&& &trimmed[trimmed.len() - 3..] == "```";
										if !is_code_comment {
											new_content += "\n";
											new_content += &code_buffer;
										}
									}
								}
								ParseCodeStatus::None => {
									// voluntary do nothing
								}
							}

							// code_buffer = String::new();
							code_status = ParseCodeStatus::None;
						}

						new_content += &comment_buffer;
						comment_buffer = String::new();
					}
					None => {
						match comment_status {
							ParseCommentStatus::Comment => {
								comment_buffer += &line.replace("-- >", "-->");
								comment_buffer += "\n";
							}
							ParseCommentStatus::None => {
								// do nothing voluntary
							}
						}
					}
				}
			}
		}

		match &comment_status {
			ParseCommentStatus::None => {
				if let Some(found) = search_code_delemiter.captures(&line) {
					if previous_line == "\n" {
						let delimiter = found[3].to_owned();

						match &code_status {
							ParseCodeStatus::Code(current_delimiter) => {
								if !code_is_closed {
									if &delimiter == current_delimiter {
										code_buffer += &found[1];
										code_buffer += &found[3];
										code_is_closed = true; // TODO : here is allocating bug !
									} else {
										code_buffer += &line;
									}
								}
							}
							ParseCodeStatus::None => {
								code_status = ParseCodeStatus::Code(delimiter.clone());

								code_buffer = String::new();
								code_buffer += &found[3];
								code_buffer += &found[4];
								code_buffer += "\n";

								code_is_closed = false;
							}
						}
					} else {
						code_buffer += &line;
						code_buffer += "\n";
					}
				} else if let ParseCodeStatus::Code(_) = code_status {
					code_buffer += &line;
					code_buffer += "\n";
				}
			}
			ParseCommentStatus::Comment => {
				// do nothing voluntary
			}
		}

		previous_line = String::from(line);
	}

	return new_content.trim().into();
}

#[derive(PartialEq)]
enum ParseCommentStatus {
	None,
	Comment,
}

#[derive(PartialEq)]
enum ParseCodeStatus {
	None,
	Code(String),
}
