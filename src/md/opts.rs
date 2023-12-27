use markdown::{CompileOptions, Constructs, Options, ParseOptions};

pub fn constructs() -> Constructs {
    Constructs {
        frontmatter: true,
        math_flow: true,
        math_text: true,
        html_flow: true,
        html_text: true,
        ..Constructs::gfm()
    }
}

pub fn parse_options() -> ParseOptions {
    ParseOptions {
        constructs: constructs(),
        ..ParseOptions::gfm()
    }
}

pub fn options() -> Options {
    Options {
        parse: parse_options(),
        compile: CompileOptions {
            allow_dangerous_html: true,
            ..CompileOptions::gfm()
        },
    }
}
