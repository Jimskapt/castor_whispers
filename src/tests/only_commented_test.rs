use crate::only_commented::only_commented;

#[test]
fn uncomment_only_one_line_commented() {
	let input = r#"<!-- Lorem ipsum dolor sit amet consectetur adipisicing elit. -->"#;

	let expected = r#"Lorem ipsum dolor sit amet consectetur adipisicing elit."#;

	assert_eq!(only_commented(input), expected);
}

#[test]
fn uncomment_only_one_line_commented_surrounded_by_commented_paragraphs() {
	let input = r#"<!--
Lorem ipsum dolor sit amet consectetur adipisicing elit. Modi sit ad eos sequi
quibusdam nesciunt, maiores rerum.
-->

<!-- Lorem ipsum dolor sit amet consectetur adipisicing elit. -->

<!--
Lorem ipsum dolor sit amet consectetur adipisicing elit. Numquam, illum nam
quis sunt ad autem et ratione ut mollitia doloribus, amet at!
-->"#;

	let expected = r#"Lorem ipsum dolor sit amet consectetur adipisicing elit. Modi sit ad eos sequi
quibusdam nesciunt, maiores rerum.

Lorem ipsum dolor sit amet consectetur adipisicing elit.

Lorem ipsum dolor sit amet consectetur adipisicing elit. Numquam, illum nam
quis sunt ad autem et ratione ut mollitia doloribus, amet at!"#;

	assert_eq!(only_commented(input), expected);
}

#[test]
fn uncomment_only_one_line_commented_surrounded_by_uncommented_paragraphs() {
	let input = r#"Lorem ipsum dolor sit amet consectetur adipisicing elit. Modi sit ad eos sequi
quibusdam nesciunt, maiores rerum.

<!-- Lorem ipsum dolor sit amet consectetur adipisicing elit. -->

Lorem ipsum dolor sit amet consectetur adipisicing elit. Numquam, illum nam
quis sunt ad autem et ratione ut mollitia doloribus, amet at!"#;

	let expected = r#"Lorem ipsum dolor sit amet consectetur adipisicing elit."#;

	assert_eq!(only_commented(input), expected);
}

#[test]
fn uncomment_only_paragraphs() {
	let input = r#"<!--
Lorem ipsum dolor sit amet consectetur adipisicing elit. Modi sit ad eos sequi
quibusdam nesciunt, maiores rerum.
-->

Lorem ipsum dolor sit amet consectetur, adipisicing elit. Hic ea alias
molestiae labore ut possimus qui iure fugit impedit rerum modi, culpa delectus
vel rem et debitis fugiat earum explicabo.

<!--
Lorem ipsum dolor sit amet consectetur adipisicing elit. Numquam, illum nam
quis sunt ad autem et ratione ut mollitia doloribus, amet at!
-->

Lorem ipsum dolor sit amet consectetur, adipisicing elit. Quam nihil harum
debitis, repudiandae accusantium ipsa totam numquam quis necessitatibus
quisquam odio optio sint commodi ea impedit voluptate, quae laboriosam aut.

<!--
Lorem ipsum dolor sit amet consectetur adipisicing elit. Doloribus eum,
consequatur est minus nemo deserunt voluptates accusantium numquam modi soluta
facere esse praesentium necessitatibus rerum quod assumenda!
-->"#;

	let expected = r#"Lorem ipsum dolor sit amet consectetur adipisicing elit. Modi sit ad eos sequi
quibusdam nesciunt, maiores rerum.

Lorem ipsum dolor sit amet consectetur adipisicing elit. Numquam, illum nam
quis sunt ad autem et ratione ut mollitia doloribus, amet at!

Lorem ipsum dolor sit amet consectetur adipisicing elit. Doloribus eum,
consequatur est minus nemo deserunt voluptates accusantium numquam modi soluta
facere esse praesentium necessitatibus rerum quod assumenda!"#;

	assert_eq!(only_commented(input), expected);
}

#[test]
fn uncomment_paragraph_with_inline_comment_commented() {
	let input = r#"<!--
Lorem ipsum dolor sit amet consectetur adipisicing elit. <!-- Modi sit ad eos -- > sequi
quibusdam nesciunt, maiores rerum.
-->"#;

	let expected = r#"Lorem ipsum dolor sit amet consectetur adipisicing elit. <!-- Modi sit ad eos --> sequi
quibusdam nesciunt, maiores rerum."#;

	assert_eq!(only_commented(input), expected);
}

#[test]
fn uncomment_paragraph_with_inline_comment_end_like_commented() {
	let input = r#"<!--
Lorem ipsum dolor sit amet consectetur adipisicing elit.
-- > Modi sit ad eos sequi quibusdam nesciunt, maiores rerum.
-->"#;

	let expected = r#"Lorem ipsum dolor sit amet consectetur adipisicing elit.
--> Modi sit ad eos sequi quibusdam nesciunt, maiores rerum."#;

	assert_eq!(only_commented(input), expected);
}

#[test]
fn uncomment_paragraphs_with_inline_comment_commented() {
	let input = r#"<!--
Lorem ipsum dolor sit amet consectetur adipisicing elit. <!-- Modi sit ad eos -- > sequi
quibusdam nesciunt, maiores rerum.
-->

<!--
Lorem ipsum dolor sit amet consectetur adipisicing elit. Numquam, illum nam
quis sunt ad autem et ratione ut mollitia doloribus, amet at!
-->"#;

	let expected = r#"Lorem ipsum dolor sit amet consectetur adipisicing elit. <!-- Modi sit ad eos --> sequi
quibusdam nesciunt, maiores rerum.

Lorem ipsum dolor sit amet consectetur adipisicing elit. Numquam, illum nam
quis sunt ad autem et ratione ut mollitia doloribus, amet at!"#;

	assert_eq!(only_commented(input), expected);
}

#[test]
fn uncomment_paragraphs_with_inline_multiline_comment_commented() {
	let input = r#"<!--
Lorem ipsum dolor sit amet consectetur adipisicing elit. <!-- Modi sit ad eos sequi
quibusdam nesciunt -- >, maiores rerum.
-->

<!--
Lorem ipsum dolor sit amet consectetur adipisicing elit. Numquam, illum nam
quis sunt ad autem et ratione ut mollitia doloribus, amet at!
-->"#;

	let expected = r#"Lorem ipsum dolor sit amet consectetur adipisicing elit. <!-- Modi sit ad eos sequi
quibusdam nesciunt -->, maiores rerum.

Lorem ipsum dolor sit amet consectetur adipisicing elit. Numquam, illum nam
quis sunt ad autem et ratione ut mollitia doloribus, amet at!"#;

	assert_eq!(only_commented(input), expected);
}

#[test]
fn uncomment_paragraphs_with_inline_comment_uncommented() {
	let input = r#"<!--
Lorem ipsum dolor sit amet consectetur adipisicing elit. Modi sit ad eos sequi
quibusdam nesciunt, maiores rerum.
-->

Lorem ipsum dolor sit amet consectetur, adipisicing elit. Hic ea alias
molestiae <!-- labore ut possimus --> qui iure fugit impedit rerum modi, culpa delectus
vel rem et debitis fugiat earum explicabo.

<!--
Lorem ipsum dolor sit amet consectetur adipisicing elit. Numquam, illum nam
quis sunt ad autem et ratione ut mollitia doloribus, amet at!
-->"#;

	let expected = r#"Lorem ipsum dolor sit amet consectetur adipisicing elit. Modi sit ad eos sequi
quibusdam nesciunt, maiores rerum.

Lorem ipsum dolor sit amet consectetur adipisicing elit. Numquam, illum nam
quis sunt ad autem et ratione ut mollitia doloribus, amet at!"#;

	assert_eq!(only_commented(input), expected);
}

#[test]
fn uncomment_paragraphs_with_inline_multiline_comment_uncommented() {
	let input = r#"<!--
Lorem ipsum dolor sit amet consectetur adipisicing elit. Modi sit ad eos sequi
quibusdam nesciunt, maiores rerum.
-->

Lorem ipsum dolor sit amet consectetur, adipisicing elit. Hic ea alias
molestiae <!-- labore ut possimus qui iure fugit impedit rerum modi, culpa delectus
vel rem et debitis --> fugiat earum explicabo.

<!--
Lorem ipsum dolor sit amet consectetur adipisicing elit. Numquam, illum nam
quis sunt ad autem et ratione ut mollitia doloribus, amet at!
-->"#;

	let expected = r#"Lorem ipsum dolor sit amet consectetur adipisicing elit. Modi sit ad eos sequi
quibusdam nesciunt, maiores rerum.

Lorem ipsum dolor sit amet consectetur adipisicing elit. Numquam, illum nam
quis sunt ad autem et ratione ut mollitia doloribus, amet at!"#;

	assert_eq!(only_commented(input), expected);
}

#[test]
fn uncomment_paragraphs_with_inline_comment_end_like_uncommented() {
	let input = r#"<!--
Lorem ipsum dolor sit amet consectetur adipisicing elit. Modi sit ad eos sequi
quibusdam nesciunt, maiores rerum.
-->

Lorem ipsum dolor sit amet consectetur, adipisicing elit.
-- > Hic ea alias molestiaelabore ut possimus qui iure fugit impedit rerum modi, culpa delectus
vel rem et debitis fugiat earum explicabo.

<!--
Lorem ipsum dolor sit amet consectetur adipisicing elit. Numquam, illum nam
quis sunt ad autem et ratione ut mollitia doloribus, amet at!
-->"#;

	let expected = r#"Lorem ipsum dolor sit amet consectetur adipisicing elit. Modi sit ad eos sequi
quibusdam nesciunt, maiores rerum.

Lorem ipsum dolor sit amet consectetur adipisicing elit. Numquam, illum nam
quis sunt ad autem et ratione ut mollitia doloribus, amet at!"#;

	assert_eq!(only_commented(input), expected);
}

#[test]
fn uncomment_paragraphs_with_comment_block() {
	let input = r#"<!--
Lorem ipsum dolor sit amet consectetur adipisicing elit. Modi sit ad eos sequi
quibusdam nesciunt, maiores rerum.
-->

<!--
<!--
Lorem ipsum dolor sit amet consectetur adipisicing elit. Numquam, illum nam
quis sunt ad autem et ratione ut mollitia doloribus, amet at!
-- >
-->

<!--
Lorem ipsum dolor sit amet consectetur adipisicing elit. Doloribus eum,
consequatur est minus nemo deserunt voluptates accusantium numquam modi soluta
facere esse praesentium necessitatibus rerum quod assumenda!
-->"#;

	let expected = r#"Lorem ipsum dolor sit amet consectetur adipisicing elit. Modi sit ad eos sequi
quibusdam nesciunt, maiores rerum.

<!--
Lorem ipsum dolor sit amet consectetur adipisicing elit. Numquam, illum nam
quis sunt ad autem et ratione ut mollitia doloribus, amet at!
-->

Lorem ipsum dolor sit amet consectetur adipisicing elit. Doloribus eum,
consequatur est minus nemo deserunt voluptates accusantium numquam modi soluta
facere esse praesentium necessitatibus rerum quod assumenda!"#;

	assert_eq!(only_commented(input), expected);
}
