use blockchainlib::*; //look at Cargo.toml
fn main() {
    let difficulty = 0x000fffffffffffffffffffffffffffff;

    let mut block = Block::new(
        0,
        now(),
        vec![0; 32],
        0,
        "Genesis block!".to_owned(),
        difficulty,
    );

    block.mine();
    println!("Mined genesis block {:?}", &block);

    let mut last_hash = block.hash.clone();

    let mut blockchain = Blockchain {
        blocks: vec![block],
    };

    for i in 1..=10 {
        let mut block = Block::new(i, now(), last_hash, 0, "Another block".to_owned(), difficulty);

        block.mine();
        println!("Mined genesis block {:?}", &block);

        last_hash = block.hash.clone();

        blockchain.blocks.push(block);

        println!("Verify {}", &blockchain.verify());
    }
        //blockchain.blocks[3].index = 4;//output: Index mismatch 4 != 3 Verify false
        //blockchain.blocks[3].hash[8] += 1;//output: Hash mismatch 4 != 3 Verify false
        //blockchain.blocks[3].payload = "Nope".to_owned();//output: Difficulty fail Verify false
        //blockchain.blocks[3].prev_block_hash[18] = 8;//output: Difficulty fail Verify false
        //println!("Verify {}", &blockchain.verify());
}
