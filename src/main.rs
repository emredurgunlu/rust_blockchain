use blockchainlib::*; //look at Cargo.toml
fn main() {
    let block = Block::new(0, 0, vec![0; 32], 1, "Genesis Block".to_owned());
    println!("{:?}", &block);

    let h = block.hash();
    println!("{:?}", &h);
}
