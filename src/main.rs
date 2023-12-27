use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(short, long, required = true, help = "Input file or directory")]
    input: String,
    #[arg(short, long, default_value = ".", help = "Output file or directory")]
    output: Option<String>,
}

fn main() {
    let cli = Cli::parse();
    let mut tree = mdsplode::parse_file(cli.input.as_str());
    tree.traverse();
    println!("{:}", tree.to_json());
}
