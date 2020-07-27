use crate::comment::comment;

#[test]
fn nnha23zjqp6qi() {
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

	assert_eq!(comment(input), expected);
}

#[test]
fn is5kenpe6c2ipltq47m() {
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

	assert_eq!(comment(input), expected);
}

#[test]
fn iacvt746t1weu() {
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

	assert_eq!(comment(input), expected);
}

#[test]
fn bgz4euhwg5b7g3itmvyo7cd() {
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

	assert_eq!(comment(input), expected);
}

#[test]
fn t2003bxt1su887yvira2es() {
	let input = r#"Lorem ipsum dolor sit amet consectetur adipisicing elit. Modi sit ad eos sequi
quibusdam nesciunt, ```dolor sit amet consectetur adipisicing elit``` maiores
rerum."#;

	let expected = r#"<!--
Lorem ipsum dolor sit amet consectetur adipisicing elit. Modi sit ad eos sequi
quibusdam nesciunt, ```dolor sit amet consectetur adipisicing elit``` maiores
rerum.
-->"#;

	assert_eq!(comment(input), expected);
}

#[test]
fn mxxzip6b8zy0r() {
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

	assert_eq!(comment(input), expected);
}

#[test]
fn q1ncbpo6v1x5issbi3v6m() {
	let input = r#"Lorem ipsum dolor sit amet consectetur adipisicing elit. Modi sit ad eos sequi
quibusdam nesciunt, <!-- dolor sit amet consectetur adipisicing elit --> maiores
rerum."#;

	let expected = r#"<!--
Lorem ipsum dolor sit amet consectetur adipisicing elit. Modi sit ad eos sequi
quibusdam nesciunt, <!-- dolor sit amet consectetur adipisicing elit -- > maiores
rerum.
-->"#;

	assert_eq!(comment(input), expected);
}

#[test]
fn idl3u6gvf3rrwaeabwn3y() {
	let input = r#"Lorem ipsum dolor sit amet consectetur adipisicing elit.
--> Modi sit ad eos sequi quibusdam nesciunt, maiores rerum."#;

	let expected = r#"<!--
Lorem ipsum dolor sit amet consectetur adipisicing elit.
-- > Modi sit ad eos sequi quibusdam nesciunt, maiores rerum.
-->"#;

	assert_eq!(comment(input), expected);
}
