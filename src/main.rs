use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(short, long)]
    file: Option<String>,
}

fn main() {
    let cli = Cli::parse();
    let mut tree = mdsplode::parse_file(cli.file.unwrap().as_str());
    tree.traverse();
    println!("{:}", tree.to_json());
}
