use std::io::prelude::*;

fn main() {
	let args: Vec<String> = std::env::args().collect();

	if args.len() < 1 {
		panic!("Please specify the path to your input file");
	}

	let input_path = args[1].as_str();

	let mut raw = std::fs::File::open(input_path)
		.expect(&format!("Can not open file {}", input_path));
	
	let mut raws_buffer = String::new();
	raw.read_to_string(&mut raws_buffer)
		.expect(&format!("Can not read file {}", input_path));
	
	let raws = raws_buffer.as_str();

	let lines: Vec<&str> = raws.split("\n").collect();

	let mut new_content = String::new(); 

	let mut previous: Option<&str> = None;
	let mut current_is_code = false;
	for line in lines {
		if !current_is_code {
			match previous {
				Some(text) => {
					if text.trim() == "" {
						new_content.push_str("<!--\n");
					}
				},
				None => {
					new_content.push_str("<!--\n");
				}
			}

			if line.trim() == "" {
				new_content.push_str("-->\n");
			}
		}

		if line.contains("```") {
			current_is_code = !current_is_code;
		}

		new_content.push_str(line.replace("-->", "-- >").as_str());
		new_content.push_str("\n");

		previous = Some(line.trim());
	}

	println!("{}", new_content);
}
