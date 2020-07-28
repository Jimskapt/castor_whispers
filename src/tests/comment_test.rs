use crate::comment::comment;

#[test]
fn comment_only_paragraphs() {
	let input = r#"Lorem ipsum dolor sit amet consectetur adipisicing elit. Modi sit ad eos sequi
quibusdam nesciunt, maiores rerum.

Lorem ipsum dolor sit amet consectetur adipisicing elit. Numquam, illum nam
quis sunt ad autem et ratione ut mollitia doloribus, amet at!

Lorem ipsum dolor sit amet consectetur adipisicing elit. Doloribus eum,
consequatur est minus nemo deserunt voluptates accusantium numquam modi soluta
facere esse praesentium necessitatibus rerum quod assumenda!"#;

	let expected = r#"<!--
Lorem ipsum dolor sit amet consectetur adipisicing elit. Modi sit ad eos sequi
quibusdam nesciunt, maiores rerum.
-->

<!--
Lorem ipsum dolor sit amet consectetur adipisicing elit. Numquam, illum nam
quis sunt ad autem et ratione ut mollitia doloribus, amet at!
-->

<!--
Lorem ipsum dolor sit amet consectetur adipisicing elit. Doloribus eum,
consequatur est minus nemo deserunt voluptates accusantium numquam modi soluta
facere esse praesentium necessitatibus rerum quod assumenda!
-->"#;

	assert_eq!(
		comment(input, &crate::settings::Settings::default()),
		expected
	);
}

#[test]
fn comment_only_paragraphs_with_end_newline() {
	let input = r#"Lorem ipsum dolor sit amet consectetur adipisicing elit. Modi sit ad eos sequi
quibusdam nesciunt, maiores rerum.

Lorem ipsum dolor sit amet consectetur adipisicing elit. Numquam, illum nam
quis sunt ad autem et ratione ut mollitia doloribus, amet at!
"#;

	let expected = r#"<!--
Lorem ipsum dolor sit amet consectetur adipisicing elit. Modi sit ad eos sequi
quibusdam nesciunt, maiores rerum.
-->

<!--
Lorem ipsum dolor sit amet consectetur adipisicing elit. Numquam, illum nam
quis sunt ad autem et ratione ut mollitia doloribus, amet at!
-->"#;

	assert_eq!(
		comment(input, &crate::settings::Settings::default()),
		expected
	);
}

#[test]
fn comment_only_paragraphs_with_end_newlines() {
	let input = r#"Lorem ipsum dolor sit amet consectetur adipisicing elit. Modi sit ad eos sequi
quibusdam nesciunt, maiores rerum.

Lorem ipsum dolor sit amet consectetur adipisicing elit. Numquam, illum nam
quis sunt ad autem et ratione ut mollitia doloribus, amet at!



"#;

	let expected = r#"<!--
Lorem ipsum dolor sit amet consectetur adipisicing elit. Modi sit ad eos sequi
quibusdam nesciunt, maiores rerum.
-->

<!--
Lorem ipsum dolor sit amet consectetur adipisicing elit. Numquam, illum nam
quis sunt ad autem et ratione ut mollitia doloribus, amet at!
-->"#;

	assert_eq!(
		comment(input, &crate::settings::Settings::default()),
		expected
	);
}

#[test]
fn comment_paragraphs_with_code_block() {
	let input = r#"Lorem ipsum dolor sit amet consectetur adipisicing elit. Modi sit ad eos sequi
quibusdam nesciunt, maiores rerum.

```text
Lorem ipsum, dolor sit amet consectetur adipisicing elit. Aut et quam maiores,
nesciunt provident voluptates eum a repellat, sed eius fugit, ut minus est
praesentium minima odio in quisquam nostrum.

Lorem ipsum dolor sit amet, consectetur adipisicing elit. Commodi autem quae
neque, soluta quam optio vero ex necessitatibus impedit dignissimos, libero
ipsam cumque incidunt suscipit corporis exercitationem voluptatum consequatur.
```

Lorem ipsum dolor sit amet consectetur adipisicing elit. Doloribus eum,
consequatur est minus nemo deserunt voluptates accusantium numquam modi soluta
facere esse praesentium necessitatibus rerum quod assumenda!"#;

	let expected = r#"<!--
Lorem ipsum dolor sit amet consectetur adipisicing elit. Modi sit ad eos sequi
quibusdam nesciunt, maiores rerum.
-->

<!--
```text
Lorem ipsum, dolor sit amet consectetur adipisicing elit. Aut et quam maiores,
nesciunt provident voluptates eum a repellat, sed eius fugit, ut minus est
praesentium minima odio in quisquam nostrum.

Lorem ipsum dolor sit amet, consectetur adipisicing elit. Commodi autem quae
neque, soluta quam optio vero ex necessitatibus impedit dignissimos, libero
ipsam cumque incidunt suscipit corporis exercitationem voluptatum consequatur.
```
-->

<!--
Lorem ipsum dolor sit amet consectetur adipisicing elit. Doloribus eum,
consequatur est minus nemo deserunt voluptates accusantium numquam modi soluta
facere esse praesentium necessitatibus rerum quod assumenda!
-->"#;

	assert_eq!(
		comment(input, &crate::settings::Settings::default()),
		expected
	);
}

#[test]
fn comment_paragraph_with_inline_code() {
	let input = r#"Lorem ipsum dolor sit amet consectetur adipisicing elit. Modi sit ad eos sequi
quibusdam nesciunt, ```dolor sit amet consectetur adipisicing elit``` maiores
rerum."#;

	let expected = r#"<!--
Lorem ipsum dolor sit amet consectetur adipisicing elit. Modi sit ad eos sequi
quibusdam nesciunt, ```dolor sit amet consectetur adipisicing elit``` maiores
rerum.
-->"#;

	assert_eq!(
		comment(input, &crate::settings::Settings::default()),
		expected
	);
}

#[test]
fn comment_paragraphs_with_comment_block() {
	let input = r#"Lorem ipsum dolor sit amet consectetur adipisicing elit. Modi sit ad eos sequi
quibusdam nesciunt, maiores rerum.

<!--
Lorem ipsum, dolor sit amet consectetur adipisicing elit. Aut et quam maiores,
nesciunt provident voluptates eum a repellat, sed eius fugit, ut minus est
praesentium minima odio in quisquam nostrum.
-->

Lorem ipsum dolor sit amet consectetur adipisicing elit. Doloribus eum,
consequatur est minus nemo deserunt voluptates accusantium numquam modi soluta
facere esse praesentium necessitatibus rerum quod assumenda!"#;

	let expected = r#"<!--
Lorem ipsum dolor sit amet consectetur adipisicing elit. Modi sit ad eos sequi
quibusdam nesciunt, maiores rerum.
-->

<!--
<!--
Lorem ipsum, dolor sit amet consectetur adipisicing elit. Aut et quam maiores,
nesciunt provident voluptates eum a repellat, sed eius fugit, ut minus est
praesentium minima odio in quisquam nostrum.
-- >
-->

<!--
Lorem ipsum dolor sit amet consectetur adipisicing elit. Doloribus eum,
consequatur est minus nemo deserunt voluptates accusantium numquam modi soluta
facere esse praesentium necessitatibus rerum quod assumenda!
-->"#;

	assert_eq!(
		comment(input, &crate::settings::Settings::default()),
		expected
	);
}

#[test]
fn comment_paragraph_with_inline_comment() {
	let input = r#"Lorem ipsum dolor sit amet consectetur adipisicing elit. Modi sit ad eos sequi
quibusdam nesciunt, <!-- dolor sit amet consectetur adipisicing elit --> maiores
rerum."#;

	let expected = r#"<!--
Lorem ipsum dolor sit amet consectetur adipisicing elit. Modi sit ad eos sequi
quibusdam nesciunt, <!-- dolor sit amet consectetur adipisicing elit -- > maiores
rerum.
-->"#;

	assert_eq!(
		comment(input, &crate::settings::Settings::default()),
		expected
	);
}

#[test]
fn comment_paragraph_with_inline_comment_end_like() {
	let input = r#"Lorem ipsum dolor sit amet consectetur adipisicing elit.
--> Modi sit ad eos sequi quibusdam nesciunt, maiores rerum."#;

	let expected = r#"<!--
Lorem ipsum dolor sit amet consectetur adipisicing elit.
-- > Modi sit ad eos sequi quibusdam nesciunt, maiores rerum.
-->"#;

	assert_eq!(
		comment(input, &crate::settings::Settings::default()),
		expected
	);
}

#[test]
fn comment_only_paragraphs_with_simple_replacement() {
	let input = r#"Lorem ipsum dolor sit amet consectetur adipisicing elit. Modi sit ad eos sequi
quibusdam nesciunt, maiores rerum.

Lorem ipsum dolor sit amet consectetur adipisicing elit. Numquam, illum nam
quis sunt ad autem et ratione ut mollitia doloribus, amet at!

Lorem ipsum dolor sit amet consectetur adipisicing elit. Doloribus eum,
consequatur est minus nemo deserunt voluptates accusantium numquam modi soluta
facere esse praesentium necessitatibus rerum quod assumenda!"#;

	let expected = r#"<!--
TEST ipsum dolor sit amet consectetur adipisicing elit. Modi sit ad eos sequi
quibusdam nesciunt, maiores rerum.
-->

<!--
TEST ipsum dolor sit amet consectetur adipisicing elit. Numquam, illum nam
quis sunt ad autem et ratione ut mollitia doloribus, amet at!
-->

<!--
TEST ipsum dolor sit amet consectetur adipisicing elit. Doloribus eum,
consequatur est minus nemo deserunt voluptates accusantium numquam modi soluta
facere esse praesentium necessitatibus rerum quod assumenda!
-->"#;

	let mut settings = crate::settings::Settings::default();
	settings.replacements.comment = Some(vec![(String::from("Lorem"), String::from("TEST"))]);

	assert_eq!(comment(input, &settings), expected);
}

#[test]
fn comment_only_paragraphs_with_regex_replacement() {
	let input = r#"Lorem ipsum dolor sit amet consectetur adipisicing elit. Modi sit ad eos sequi
quibusdam nesciunt, maiores rerum.

Lorem ipsum dolor sit amet consectetur adipisicing elit. Numquam, illum nam
quis sunt ad autem et ratione ut mollitia doloribus, amet at!

Lorem ipsum dolor sit amet consectetur adipisicing elit. Doloribus eum,
consequatur est minus nemo deserunt voluptates accusantium numquam modi soluta
facere esse praesentium necessitatibus rerum quod assumenda!"#;

	let expected = r#"<!--
Lorem ipsum dolor sit amet consectetur TEST. Modi sit ad eos sequi NEWLINE
quibusdam nesciunt, maiores rerum.
-->

<!--
Lorem ipsum dolor sit amet consectetur TEST. Numquam, illum nam NEWLINE
quis sunt ad autem et ratione ut mollitia doloribus, amet at!
-->

<!--
Lorem ipsum dolor sit amet consectetur TEST. Doloribus eum, NEWLINE
consequatur est minus nemo deserunt voluptates accusantium numquam modi soluta
facere esse praesentium necessitatibus rerum quod assumenda!
-->"#;

	let mut settings = crate::settings::Settings::default();
	settings.replacements.comment = Some(vec![(
		String::from("adipisicing elit\\. (.+)"),
		String::from("TEST. $1 NEWLINE"),
	)]);

	assert_eq!(comment(input, &settings), expected);
}
