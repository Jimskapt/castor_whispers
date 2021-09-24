# Castor Whispers

An utility to work with markdown text files.

## Usage

```cmd
USAGE:
    castor_whispers.exe [SUBCOMMAND]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    comment           Comment all paragraphs
    help              Prints this message or the help of the given subcommand(s)
    only-commented    Get only commented paragraphs
```

File : input.md

````markdown
# Title

## Sub-title

Lorem ipsum dolor sit amet, consectetur adipiscing elit. Aliquam vel felis et metus tempor consequat. Cras sit amet condimentum est, ac mattis velit. Mauris malesuada mollis neque, in convallis est varius eu.
<!-- this is a random comment -->
Pellentesque dictum tempus velit non faucibus. Vestibulum hendrerit, diam et malesuada sollicitudin, neque enim ullamcorper mauris, quis dictum magna tortor id nisl.

```rust
fn main() {
    print_example():
}

fn print_example() {
    println!("This is an example.");
}
```

Donec maximus gravida ipsum ac sollicitudin. Cras feugiat ac diam suscipit ullamcorper. Aenean condimentum id diam ut mattis. Mauris diam odio, gravida id purus ut, gravida tincidunt diam. Donec et egestas augue. Maecenas non felis sed ipsum dictum auctor. Quisque at tellus eros. Aenean dictum massa et varius luctus.

````

Then, type in your CLI :

```cmd
castor-whispers comment input.md
```

It will print on your screen :

````markdown
<!--
# Title
-->

<!--
## Sub-title
-->

<!--
Lorem ipsum dolor sit amet, consectetur adipiscing elit. Aliquam vel felis et metus tempor consequat. Cras sit amet condimentum est, ac mattis velit. Mauris malesuada mollis neque, in convallis est varius eu.
<!-- this is a random comment -- >
Pellentesque dictum tempus velit non faucibus. Vestibulum hendrerit, diam et malesuada sollicitudin, neque enim ullamcorper mauris, quis dictum magna tortor id nisl.
-->

<!--
```rust
fn main() {
    print_example():
}

fn print_example() {
    println!("This is an example.");
}
```
-->

<!--
Donec maximus gravida ipsum ac sollicitudin. Cras feugiat ac diam suscipit ullamcorper. Aenean condimentum id diam ut mattis. Mauris diam odio, gravida id purus ut, gravida tincidunt diam. Donec et egestas augue. Maecenas non felis sed ipsum dictum auctor. Quisque at tellus eros. Aenean dictum massa et varius luctus.
-->

````

You can type this command in your CLI to save this result in `output.md` file :

```cmd
castor-whispers comment input.md > output.md
```

## Settings

Please read `struct Settings` in `/src/settings.rs` file to write your `./castor_whispers.conf.toml` config file.
An example exists in `/castor_whispers.conf.toml` in this repository.
