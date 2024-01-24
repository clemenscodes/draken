use clap::Parser;

#[derive(Parser)]
#[command(name = env!("CARGO_PKG_NAME"))]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(short, long)]
    fen: Option<String>,
}

fn main() {
    let cli = Cli::parse();
    println!("fen: {:?}", cli.fen.as_deref());
}
