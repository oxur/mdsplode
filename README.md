# mdsplode

[![][build-badge]][build]
[![][crate-badge]][crate]
[![][tag-badge]][tag]
[![][docs-badge]][docs]

[![][logo]][logo-large]

*A CLI tool for exploding and serialising Markdown files (metadata, AST, rendered parts, and rendered whole)*

## About

This tool came about as a result of a pressing need in some LFE projects that needed finer-grained processing of Markdown files for publishing projects.

## Dependencies

This tool filters using `jq` query strings and this requires that you let `mdsplode` know where your `jq` and other dependent libs are installed. If you're on a mac and `jq` was installed with homebrew, you can get the lib directories with the following:

```shell
brew list jq
```

```shell
/opt/homebrew/Cellar/jq/1.6/bin/jq
/opt/homebrew/Cellar/jq/1.6/include/ (2 files)
/opt/homebrew/Cellar/jq/1.6/lib/libjq.1.dylib
/opt/homebrew/Cellar/jq/1.6/lib/ (2 other files)
/opt/homebrew/Cellar/jq/1.6/share/doc/ (4 files)
/opt/homebrew/Cellar/jq/1.6/share/man/man1/jq.1
```

```shell
brew list oniguruma
```

```shell
/opt/homebrew/Cellar/oniguruma/6.9.7.1/bin/onig-config
/opt/homebrew/Cellar/oniguruma/6.9.7.1/include/ (2 files)
/opt/homebrew/Cellar/oniguruma/6.9.7.1/lib/libonig.5.dylib
/opt/homebrew/Cellar/oniguruma/6.9.7.1/lib/pkgconfig/oniguruma.pc
/opt/homebrew/Cellar/oniguruma/6.9.7.1/lib/ (2 other files)
```

Use these to set the `JQ_LIB_DIR` environment variable; in this case:

```shell
export JQ_LIB_DIR=/opt/homebrew/Cellar/jq/1.6/lib
export ONIG_LIB_DIR=/opt/homebrew/Cellar/oniguruma/6.9.7.1/lib
```

With these set to the appropriate values for your system, running `make build` should result in a usable `./bin/mdplode`.

## Usage

```shell
./bin/mdsplode \
    --input ./tests/data/learn.md \
    --query '.children.nodes'
```

## License

Copyright © 2023-2024, Oxur Group

Apache License, Version 2.0

[//]: ---Named-Links---

[logo]: https://raw.githubusercontent.com/oxur/mdsplode/main/resources/images/logo-small.jpg
[logo-large]: https://raw.githubusercontent.com/oxur/mdsplode/main/resources/images/logo.jpg
[build]: https://github.com/oxur/mdsplode/actions/workflows/cicd.yml
[build-badge]: https://github.com/oxur/mdsplode/actions/workflows/cicd.yml/badge.svg
[crate]: https://crates.io/crates/mdsplode
[crate-badge]: https://img.shields.io/crates/v/mdsplode.svg
[docs]: https://docs.rs/mdsplode/
[docs-badge]: https://img.shields.io/badge/rust-documentation-blue.svg
[tag-badge]: https://img.shields.io/github/tag/oxur/mdsplode.svg
[tag]: https://github.com/oxur/mdsplode/tags
