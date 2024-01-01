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

View all of the exploded parsed data:

```shell
./bin/mdsplode \
    --input ./tests/data/learn.md \
    --pretty
```

See a filtered subset, for example, the exploded metadata for a particular Markdown heading:

```shell
./bin/mdsplode \
    --input ./tests/data/learn.md \
    --pretty \
    --query '.children.nodes[] | select(((.depth == 3) and .name == "heading") and .source == "Getting Started")'
```

**Important!**: When using `--pretty`, only `jq` queries which result in valid JSON should be used; anything else will result in JSON parsing errors (since the use of `--pretty` requires a re-parsing to JSON for the post-query results).

Get the HTML for all headings of depth 3:

```shell
./bin/mdsplode \
    --input ./tests/data/learn.md \
    --query '.children.nodes[] | select((.depth == 3) and .name == "heading") | .children.nodes[].html'
```

That command doesn't produce valid JSON, so the `--pretty` wasn't used (to avoid errors and because it simply wasn't applicable).

Query to pull out the front matter:

```shell:
./bin/mdsplode \
    --input ./tests/data/learn.md \
    --query '.children.nodes[] | select(.name == "toml") | .json'
```

Since that field is serialised JSON, we can pipe it to `jq` as a raw string and then again to parse it as JSON and pretty-print it:

```shell
echo `!!` | jq -r . | jq .frontmatter
```

```json
{
  "in_search_index": true,
  "title": "Learn",
  "extra": {
    "long_description": "Learning LFE must be taken in three tracks: learning the syntax particular to a Lisp on the Erlang VM, with all its support for pattern matching, Erlang-style arities, etc.; learning the ins-and-outs of BEAM languages and OTP; and finally, more deeply exploring the Lisp heritage of LFE. This multi-pronged approach is the path to success.",
    "long_title": "Resources for Learning LFE"
  }
}
```

Note that `mdsplode` also supports operating on directories (for processing multiple files in one go).

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
