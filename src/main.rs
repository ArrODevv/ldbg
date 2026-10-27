use ldbg::config::Config;

fn main() {
    let c = Config::default();
    println!("Hello, world!");
    println!("{:#?}", c);
}
