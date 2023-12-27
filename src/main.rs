fn main() {
    let file = "./workbench/learn.md";
    let mut tree = mdsplode::parse_file(file);
    tree.traverse();
    println!("{:}", tree.to_json());
}
