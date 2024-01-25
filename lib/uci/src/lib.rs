use clap::Parser;

#[derive(Parser)]
#[command(name = "draken")]
#[command(author, version, about, long_about = None)]
pub struct UniversalChessInterface {
    #[arg(short, long)]
    pub fen: Option<String>,
}

pub fn get_uci() -> UniversalChessInterface {
    UniversalChessInterface::parse()
}
