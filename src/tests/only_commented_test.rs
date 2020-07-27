use crate::only_commented::only_commented;

#[test]
fn s2oqx1ywkbp() {
	let input = r#"<!-- Lorem ipsum dolor sit amet consectetur adipisicing elit. -->"#;

	let expected = r#"Lorem ipsum dolor sit amet consectetur adipisicing elit."#;

	assert_eq!(only_commented(input), expected);
}

#[test]
fn jhhcvv3fhjbxqvah() {
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
fn llepjnztbo4gii() {
	let input = r#"Lorem ipsum dolor sit amet consectetur adipisicing elit. Modi sit ad eos sequi
quibusdam nesciunt, maiores rerum.

<!-- Lorem ipsum dolor sit amet consectetur adipisicing elit. -->

Lorem ipsum dolor sit amet consectetur adipisicing elit. Numquam, illum nam
quis sunt ad autem et ratione ut mollitia doloribus, amet at!"#;

	let expected = r#"Lorem ipsum dolor sit amet consectetur adipisicing elit."#;

	assert_eq!(only_commented(input), expected);
}

#[test]
fn xtx2kvke4y5g0qgo3w4irn() {
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
fn j3dtvjalms2n3cpub53qn0x() {
	let input = r#"<!--
Lorem ipsum dolor sit amet consectetur adipisicing elit. <!-- Modi sit ad eos -- > sequi
quibusdam nesciunt, maiores rerum.
-->"#;

	let expected = r#"Lorem ipsum dolor sit amet consectetur adipisicing elit. <!-- Modi sit ad eos --> sequi
quibusdam nesciunt, maiores rerum."#;

	assert_eq!(only_commented(input), expected);
}

#[test]
fn w017wtpi12ijdaiws7() {
	let input = r#"<!--
Lorem ipsum dolor sit amet consectetur adipisicing elit.
-- > Modi sit ad eos sequi quibusdam nesciunt, maiores rerum.
-->"#;

	let expected = r#"Lorem ipsum dolor sit amet consectetur adipisicing elit.
--> Modi sit ad eos sequi quibusdam nesciunt, maiores rerum."#;

	assert_eq!(only_commented(input), expected);
}

#[test]
fn a218ct6vn7zddki31opmqf() {
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
fn rwuymdidm02nkovtyetwm() {
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
fn n7ejvj7m1xxsfryw() {
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
fn edlcaaypyh4uirwoqamm() {
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
fn n5lt0jjdp5rvj8u() {
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
fn dvcc7xp8jqwwqyutgu8ww6() {
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

#[test]
fn sh1dxxekfi6h() {
	let input = r#"<!--
Lorem ipsum dolor sit amet consectetur `<!--` adipisicing elit. Modi sit ad eos sequi
quibusdam nesciunt, maiores rerum.
-->

Lorem ipsum dolor sit amet consectetur, adipisicing elit.
Hic ea alias molestiaelabore ut possimus qui iure fugit impedit rerum modi, culpa delectus
vel rem et debitis fugiat earum explicabo.

<!--
Lorem ipsum dolor sit amet consectetur adipisicing elit. Numquam, illum nam
quis sunt ad autem et ratione ut mollitia doloribus, amet at!
-->"#;

	let expected = r#"Lorem ipsum dolor sit amet consectetur `<!--` adipisicing elit. Modi sit ad eos sequi
quibusdam nesciunt, maiores rerum.

Lorem ipsum dolor sit amet consectetur adipisicing elit. Numquam, illum nam
quis sunt ad autem et ratione ut mollitia doloribus, amet at!"#;

	assert_eq!(only_commented(input), expected);
}

#[test]
fn nlwa4beninzn3gr0() {
	let input = r#"<!--
Lorem ipsum dolor sit amet consectetur adipisicing elit. Modi sit ad eos sequi
quibusdam nesciunt, maiores rerum.
-->

Lorem ipsum, dolor sit amet consectetur adipisicing elit. Laborum ab fugit molestiae.

<!--
```
Lorem ipsum, dolor sit amet consectetur adipisicing elit. Aliquid laboriosam
fuga laborum alias iste quod, dolores, enim praesentium dolorum aperiam fugiat?
```
-->

```
Lorem ipsum dolor sit amet consectetur adipisicing elit. Numquam, illum nam
quis sunt ad autem et ratione ut mollitia doloribus, amet at!
```

<!--
Lorem ipsum dolor sit amet consectetur adipisicing elit. Doloribus eum,
consequatur est minus nemo deserunt voluptates accusantium numquam modi soluta
facere esse praesentium necessitatibus rerum quod assumenda!
-->
"#;

	let expected = r#"Lorem ipsum dolor sit amet consectetur adipisicing elit. Modi sit ad eos sequi
quibusdam nesciunt, maiores rerum.

```
Lorem ipsum, dolor sit amet consectetur adipisicing elit. Aliquid laboriosam
fuga laborum alias iste quod, dolores, enim praesentium dolorum aperiam fugiat?
```

Lorem ipsum dolor sit amet consectetur adipisicing elit. Doloribus eum,
consequatur est minus nemo deserunt voluptates accusantium numquam modi soluta
facere esse praesentium necessitatibus rerum quod assumenda!"#;

	assert_eq!(only_commented(input), expected);
}

#[test]
fn qotw30gqbwc2l() {
	let input = r#"<!--
Lorem ipsum dolor sit amet consectetur adipisicing elit. Modi sit ad eos sequi
quibusdam nesciunt, maiores rerum.
-->

Lorem ipsum, dolor sit amet consectetur adipisicing elit. Laborum ab fugit molestiae.

```
Lorem ipsum, dolor sit amet consectetur adipisicing elit. Aliquid laboriosam
fuga laborum alias iste quod, dolores, enim praesentium dolorum aperiam fugiat?
```

<!--
Lorem ipsum dolor sit amet consectetur adipisicing elit. Doloribus eum,
consequatur est minus nemo deserunt voluptates accusantium numquam modi soluta
facere esse praesentium necessitatibus rerum quod assumenda!
-->
"#;

	let expected = r#"Lorem ipsum dolor sit amet consectetur adipisicing elit. Modi sit ad eos sequi
quibusdam nesciunt, maiores rerum.

```
Lorem ipsum, dolor sit amet consectetur adipisicing elit. Aliquid laboriosam
fuga laborum alias iste quod, dolores, enim praesentium dolorum aperiam fugiat?
```

Lorem ipsum dolor sit amet consectetur adipisicing elit. Doloribus eum,
consequatur est minus nemo deserunt voluptates accusantium numquam modi soluta
facere esse praesentium necessitatibus rerum quod assumenda!"#;

	assert_eq!(only_commented(input), expected);
}

#[test]
fn jjzyqt3axukxb677dh5ak30k() {
	let input = r#"<!--
Lorem ipsum dolor sit amet consectetur adipisicing elit. Modi sit ad eos sequi
quibusdam nesciunt, maiores rerum.
-->

Lorem ipsum, dolor sit amet consectetur adipisicing elit. Laborum ab fugit molestiae.

````rust
Lorem ipsum, dolor sit amet consectetur adipisicing elit.

```js
Lorem ipsum dolor sit amet consectetur adipisicing elit.
Aliquam, consequatur quas quasi, eum laborum optio explicabo placeat ratione autem
qui officiis nostrum assumenda?
```

Aliquid laboriosam fuga laborum alias iste quod, dolores, enim praesentium
dolorum aperiam fugiat?
````

<!--
Lorem ipsum dolor sit amet consectetur adipisicing elit. Doloribus eum,
consequatur est minus nemo deserunt voluptates accusantium numquam modi soluta
facere esse praesentium necessitatibus rerum quod assumenda!
-->
"#;

	let expected = r#"Lorem ipsum dolor sit amet consectetur adipisicing elit. Modi sit ad eos sequi
quibusdam nesciunt, maiores rerum.

````rust
Lorem ipsum, dolor sit amet consectetur adipisicing elit.

```js
Lorem ipsum dolor sit amet consectetur adipisicing elit.
Aliquam, consequatur quas quasi, eum laborum optio explicabo placeat ratione autem
qui officiis nostrum assumenda?
```

Aliquid laboriosam fuga laborum alias iste quod, dolores, enim praesentium
dolorum aperiam fugiat?
````

Lorem ipsum dolor sit amet consectetur adipisicing elit. Doloribus eum,
consequatur est minus nemo deserunt voluptates accusantium numquam modi soluta
facere esse praesentium necessitatibus rerum quod assumenda!"#;

	assert_eq!(only_commented(input), expected);
}

#[test]
fn xqdnrai1rjn75ojx3uaxd8sz() {
	let input = r#"<!--
Lorem ipsum dolor sit amet consectetur adipisicing elit. Modi sit ad eos sequi
quibusdam nesciunt, maiores rerum.
-->

Lorem ipsum, dolor sit amet consectetur adipisicing elit. Laborum ab fugit molestiae.

```
Lorem ipsum, dolor sit amet consectetur adipisicing elit. Aliquid laboriosam
fuga laborum alias iste quod, dolores, enim praesentium dolorum aperiam fugiat?
```

<!--
Lorem ipsum dolor sit amet consectetur adipisicing elit. Doloribus eum,
consequatur est ```minus nemo `deserunt` voluptates``` accusantium numquam modi soluta
facere esse praesentium necessitatibus rerum quod assumenda!
-->
"#;

	let expected = r#"Lorem ipsum dolor sit amet consectetur adipisicing elit. Modi sit ad eos sequi
quibusdam nesciunt, maiores rerum.

```
Lorem ipsum, dolor sit amet consectetur adipisicing elit. Aliquid laboriosam
fuga laborum alias iste quod, dolores, enim praesentium dolorum aperiam fugiat?
```

Lorem ipsum dolor sit amet consectetur adipisicing elit. Doloribus eum,
consequatur est ```minus nemo `deserunt` voluptates``` accusantium numquam modi soluta
facere esse praesentium necessitatibus rerum quod assumenda!"#;

	assert_eq!(only_commented(input), expected);
}

#[test]
fn erzkeiboawu7khviz() {
	let input = r#"<!--
Lorem ipsum dolor sit amet consectetur adipisicing elit. Modi sit ad eos sequi
quibusdam nesciunt, maiores rerum.
-->

Lorem ipsum, dolor sit amet consectetur adipisicing elit. Laborum ab fugit molestiae.


```
Lorem ipsum, dolor sit amet consectetur adipisicing elit. Aliquid laboriosam
fuga laborum alias iste quod, dolores, enim praesentium dolorum aperiam fugiat?
```


<!--
Lorem ipsum dolor sit amet consectetur adipisicing elit. Doloribus eum,
consequatur est minus nemo deserunt voluptates accusantium numquam modi soluta
facere esse praesentium necessitatibus rerum quod assumenda!
-->
"#;

	let expected = r#"Lorem ipsum dolor sit amet consectetur adipisicing elit. Modi sit ad eos sequi
quibusdam nesciunt, maiores rerum.

```
Lorem ipsum, dolor sit amet consectetur adipisicing elit. Aliquid laboriosam
fuga laborum alias iste quod, dolores, enim praesentium dolorum aperiam fugiat?
```

Lorem ipsum dolor sit amet consectetur adipisicing elit. Doloribus eum,
consequatur est minus nemo deserunt voluptates accusantium numquam modi soluta
facere esse praesentium necessitatibus rerum quod assumenda!"#;

	assert_eq!(only_commented(input), expected);
}

#[test]
fn bvewmylydb4xszilu7ra() {
	let input = r#"<!--
Lorem ipsum dolor sit amet consectetur adipisicing elit. Modi sit ad eos sequi
quibusdam nesciunt, maiores rerum.
-->

Lorem ipsum, dolor sit amet consectetur adipisicing elit. Laborum ab fugit molestiae.
```
Lorem ipsum, dolor sit amet consectetur adipisicing elit. Aliquid laboriosam
fuga laborum alias iste quod, dolores, enim praesentium dolorum aperiam fugiat?
```

<!--
Lorem ipsum dolor sit amet consectetur adipisicing elit. Doloribus eum,
consequatur est minus nemo deserunt voluptates accusantium numquam modi soluta
facere esse praesentium necessitatibus rerum quod assumenda!
-->
"#;

	let expected = r#"Lorem ipsum dolor sit amet consectetur adipisicing elit. Modi sit ad eos sequi
quibusdam nesciunt, maiores rerum.

Lorem ipsum dolor sit amet consectetur adipisicing elit. Doloribus eum,
consequatur est minus nemo deserunt voluptates accusantium numquam modi soluta
facere esse praesentium necessitatibus rerum quod assumenda!"#;

	assert_eq!(only_commented(input), expected);
}

#[test]
fn ucumt4avhmwv1yh7riri() {
	let input = r#"<!--
Lorem ipsum dolor sit amet consectetur adipisicing elit. Modi sit ad eos sequi
quibusdam nesciunt, maiores rerum.
-->

Lorem ipsum, dolor sit amet consectetur adipisicing elit. Laborum ab fugit molestiae.```
Lorem ipsum, dolor sit amet consectetur adipisicing elit. Aliquid laboriosam
fuga laborum alias iste quod, dolores, enim praesentium dolorum aperiam fugiat?
```

<!--
Lorem ipsum dolor sit amet consectetur adipisicing elit. Doloribus eum,
consequatur est minus nemo deserunt voluptates accusantium numquam modi soluta
facere esse praesentium necessitatibus rerum quod assumenda!
-->
"#;

	let expected = r#"Lorem ipsum dolor sit amet consectetur adipisicing elit. Modi sit ad eos sequi
quibusdam nesciunt, maiores rerum.

Lorem ipsum dolor sit amet consectetur adipisicing elit. Doloribus eum,
consequatur est minus nemo deserunt voluptates accusantium numquam modi soluta
facere esse praesentium necessitatibus rerum quod assumenda!"#;

	assert_eq!(only_commented(input), expected);
}

#[test]
fn ip3b2gmzgq4suk1nbd() {
	let input = r#"<!--
Lorem ipsum dolor sit amet consectetur adipisicing elit. Modi sit ad eos sequi
quibusdam nesciunt, maiores rerum.
-->

```
Lorem ipsum, dolor sit amet consectetur adipisicing elit. Aliquid laboriosam
fuga laborum alias iste quod, dolores, enim praesentium dolorum aperiam fugiat?
```
Lorem ipsum, dolor sit amet consectetur adipisicing elit. Laborum ab fugit molestiae.

<!--
Lorem ipsum dolor sit amet consectetur adipisicing elit. Doloribus eum,
consequatur est minus nemo deserunt voluptates accusantium numquam modi soluta
facere esse praesentium necessitatibus rerum quod assumenda!
-->
"#;

	let expected = r#"Lorem ipsum dolor sit amet consectetur adipisicing elit. Modi sit ad eos sequi
quibusdam nesciunt, maiores rerum.

Lorem ipsum dolor sit amet consectetur adipisicing elit. Doloribus eum,
consequatur est minus nemo deserunt voluptates accusantium numquam modi soluta
facere esse praesentium necessitatibus rerum quod assumenda!"#;

	assert_eq!(only_commented(input), expected);
}

#[test]
fn kif5p64s8qk7tbt() {
	let input = r#"<!--
Lorem ipsum dolor sit amet consectetur adipisicing elit. Modi sit ad eos sequi
quibusdam nesciunt, maiores rerum.
-->

```
Lorem ipsum, dolor sit amet consectetur adipisicing elit. Aliquid laboriosam
fuga laborum alias iste quod, dolores, enim praesentium dolorum aperiam fugiat?
```Lorem ipsum, dolor sit amet consectetur adipisicing elit. Laborum ab fugit molestiae.

<!--
Lorem ipsum dolor sit amet consectetur adipisicing elit. Doloribus eum,
consequatur est minus nemo deserunt voluptates accusantium numquam modi soluta
facere esse praesentium necessitatibus rerum quod assumenda!
-->
"#;

	let expected = r#"Lorem ipsum dolor sit amet consectetur adipisicing elit. Modi sit ad eos sequi
quibusdam nesciunt, maiores rerum.

Lorem ipsum dolor sit amet consectetur adipisicing elit. Doloribus eum,
consequatur est minus nemo deserunt voluptates accusantium numquam modi soluta
facere esse praesentium necessitatibus rerum quod assumenda!"#;

	assert_eq!(only_commented(input), expected);
}

#[test]
fn go8vv3qoh8i164eub() {
	let input = r#"<!--
Lorem ipsum dolor sit amet consectetur adipisicing elit. Modi sit ad eos sequi
quibusdam nesciunt, maiores rerum.
-->

```
Lorem ipsum, dolor sit amet consectetur adipisicing elit. Aliquid laboriosam
fuga laborum alias iste quod, dolores, enim praesentium dolorum aperiam fugiat?
```

<!--
Lorem ipsum dolor sit amet consectetur adipisicing elit. Doloribus eum,
consequatur est minus nemo deserunt voluptates accusantium numquam modi soluta
facere esse praesentium necessitatibus rerum quod assumenda!
"#;

	let expected = r#"Lorem ipsum dolor sit amet consectetur adipisicing elit. Modi sit ad eos sequi
quibusdam nesciunt, maiores rerum.

Lorem ipsum dolor sit amet consectetur adipisicing elit. Doloribus eum,
consequatur est minus nemo deserunt voluptates accusantium numquam modi soluta
facere esse praesentium necessitatibus rerum quod assumenda!"#;

	assert_eq!(only_commented(input), expected);
}

#[test]
fn dl0s3fx24mlaqh() {
	let input = r#"Lorem ipsum dolor sit amet consectetur adipisicing elit. Modi sit ad eos sequi
quibusdam nesciunt, maiores rerum.

```
Lorem ipsum, dolor sit amet consectetur adipisicing elit. Aliquid laboriosam
fuga laborum alias iste quod, dolores, enim praesentium dolorum aperiam fugiat?
```

<!--
Lorem ipsum dolor sit amet consectetur adipisicing elit. Doloribus eum,
consequatur est minus nemo deserunt voluptates accusantium numquam modi soluta
facere esse praesentium necessitatibus rerum quod assumenda!
-->
"#;

	let expected = r#"Lorem ipsum dolor sit amet consectetur adipisicing elit. Doloribus eum,
consequatur est minus nemo deserunt voluptates accusantium numquam modi soluta
facere esse praesentium necessitatibus rerum quod assumenda!"#;

	assert_eq!(only_commented(input), expected);
}

#[test]
fn dsc4ujh53vg() {
	let input = r#"<!--
Lorem ipsum dolor sit amet consectetur adipisicing elit. Modi sit ad eos sequi
quibusdam nesciunt, maiores rerum.
-->

```
Lorem ipsum dolor sit amet consectetur adipisicing elit. Numquam, illum nam
quis sunt ad autem et ratione ut mollitia doloribus, amet at!
```

<!--
Lorem ipsum dolor sit amet consectetur adipisicing elit. Doloribus eum,
consequatur est minus nemo deserunt voluptates accusantium numquam modi soluta
facere esse praesentium necessitatibus rerum quod assumenda!
-->
"#;

	let expected = r#"Lorem ipsum dolor sit amet consectetur adipisicing elit. Modi sit ad eos sequi
quibusdam nesciunt, maiores rerum.

```
Lorem ipsum, dolor sit amet consectetur adipisicing elit. Aliquid laboriosam
fuga laborum alias iste quod, dolores, enim praesentium dolorum aperiam fugiat?
```

Lorem ipsum dolor sit amet consectetur adipisicing elit. Doloribus eum,
consequatur est minus nemo deserunt voluptates accusantium numquam modi soluta
facere esse praesentium necessitatibus rerum quod assumenda!"#;

	assert_eq!(only_commented(input), expected);
}
