use clap::Parser;

const STDOUT: &str = "stdout";

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(short, long, required = true, help = "Input file or directory")]
    input: String,
    #[arg(
        short,
        long,
        default_value = STDOUT,
        help = "Output file or directory"
    )]
    output: String,
    #[arg(short, long, help = "Optionally filter on a JSONpath selector")]
    selector: Option<String>,
}

fn main() {
    let cli = Cli::parse();
    let tree = mdsplode::parse_file(cli.input.as_str());
    let json = tree.to_json();
    // if match cli.selector {
    //     Some(sel) => {

    //     }
    // }
    match cli.output.as_str() {
        STDOUT => println!("{:}", json),
        _ => unimplemented!(),
    }
}
