use clap::Parser;

#[derive(Parser)]
#[command(name = env!("CARGO_PKG_NAME"))]
#[command(author, version, about, long_about = None)]
struct UniversalChessInterface {
    #[arg(short, long)]
    fen: Option<String>,
}

fn main() {
    let uci = UniversalChessInterface::parse();
    println!("fen: {:?}", uci.fen.as_deref());
}
