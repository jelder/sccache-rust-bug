use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Your name
    #[arg(short, long)]
    name: Option<String>,
}

fn main() {
    let cli = Cli::parse();

    if let Some(name) = cli.name {
        println!("Hello, {}!", name);
    } else {
        println!("Hello, world!");
    }
}
