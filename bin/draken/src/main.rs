fn main() {
    let uci = uci::get_uci();
    println!("fen: {:?}", uci.fen.as_deref());
}
