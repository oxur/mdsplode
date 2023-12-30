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

The project is written in Rust and depends upon that toolchain as well as the usual gcc toolchain. In addition, the library files for `jq` and `oniguruma` are needed. For instructions on how to install these, see [Supporting jq Queries](./docs/jq-support.md).

## Usage

```shell
./bin/mdsplode \
    --input ./tests/data/learn.md \
    --query '.children.nodes' \
    --pretty
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
