use std::io::prelude::*;

fn main() {
	let args: Vec<String> = std::env::args().collect();

	println!("Castor Whispers 1.0.0");

	if args.len() < 1 {
		panic!("Please specify path to your file");
	}

	let input_path = args[1].as_str();

	let output_path = if args.len() > 2 {
		args[2].as_str()
	} else {
		input_path
	};

	println!("Processing {} to {} ...", input_path, output_path);

	let mut raw = std::fs::File::open(input_path)
		.expect(&format!("Can not open file {}", input_path));
	
	let mut raws_buffer = String::new();
	raw.read_to_string(&mut raws_buffer)
		.expect(&format!("Can not read file {}", input_path));
	
	let raws = raws_buffer.as_str();

	let lines: Vec<&str> = raws.split("\n").collect();

	let mut new_content = String::new(); 

	let mut previous = None;
	let mut current_is_code = false;
	for line in lines {
		if !current_is_code {
			match previous {
				Some(text) => {
					if text == "" {
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

	std::fs::write(output_path, new_content.as_str())
		.expect(&format!("Can not write in file {}", output_path));
}
