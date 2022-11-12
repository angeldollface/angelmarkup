# ANGEL MARKUP LANGUAGE :performing_arts: :guitar: :ribbon:

***Simple data storage with no drama. :performing_arts: :guitar: :ribbon:***

![GitHub CI](https://github.com/angeldollface/angelmarkup/actions/workflows/rust.yml/badge.svg)

## ABOUT

I needed a data-storage format which allows comments and doesn't like drama. **A**NGEL **M**ARKUP **L**ANGUAGE is that format. This implementation is the compiler for this data format written in Rust. :heart:

## INSTALLATION

You should have the following tools installed and available from the command line:

- Rust
- Git

To install ***Angel Markup***, there are two options:

- Install the latest cutting-edge version from GitHub:

```bash
$ cargo install --git https://github.com/angeldollface/angelmarkup
```

- Install the latest stable version from Rust's package registry:

```bash
$ cargo install angelmarkup
```

This should make the `amlc` binary available from the command line.

## USAGE

### COMMAND LINE

Once you have correctly installed ***ANGELMARKUP***, you should have the `amlc` binary available from the command line.
You can use the compiler in the following ways:

- Compile an ***ANGELMARKUP*** file to YAML:

```bash
$ amlc --inf sample.aml --ouf sample.yml --yml
# OR
$ amlc -i sample.aml -o sample.yml -y
```

- Compile an ***ANGELMARKUP*** file to TOML:

```bash
$ amlc --inf sample.aml --ouf sample.toml --tml
# OR
$ amlc -i sample.aml -o sample.toml -t
```

- Compile an ***ANGELMARKUP*** file to JSON:

```bash
$ amlc --inf sample.aml --ouf sample.json --jsn
# OR
$ amlc -i sample.aml -o sample.json -j
```

- Get some version info about the compiler:

```bash
$ amlc --version
# OR
$ amlc -v
```

- Get some help info about the compiler:

```bash
$ amlc --help
# OR
$ amlc -h
```

### IN RUST PROJECTS

To use ***ANGELMARKUP*** in your projects, add this to your `Cargo.toml`:

```TOML
[dependencies]
angelmarkup = "1.1.0"
```

To import ***ANGELMARKUP***'s functions, put this line of code inside your Rust code:

```Rust
use angelmarkup::*;
```

To refer to ***ANGELMARKUP***'s functions and structs, please click [here](https://github.com/angeldollface/angelmarkup/blob/main/src/lib.rs).

## CHANGELOG

### Version 1.0.0

- Initial release.
- Upload to GitHub.

### Version 1.1.0

- Implemented a small linter.
- Implemented compilation from AML to JSON.
- Implemented compilation from AML to TOML.
- Implemented compilation from AML to YAML.
- Improved the CLI with the help of `cleasy.`
- Implemented a `Result` for the `serialize` method.
- Implemented an error `enum` for Angelmarkup Language code.

### Version 1.2.0

- The Doll Update.
- Updated, published, and uploaded under my new name.

## NOTE

- *Angel Markup Language :performing_arts: :guitar: :ribbon:* by Alexander Abraham :black_heart: a.k.a. *"Angel Dollface" :dolls: :ribbon:*
- Licensed under the MIT license.
