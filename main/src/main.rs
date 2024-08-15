use core::blockchain;
use std::thread;
use std::time::Duration;

fn main() {
    println!("Hello to Jeffy BlockChain!");
    let mut chain = blockchain::BlockChain::new_blockchain();

    println!("Start mining ...");
    thread::sleep(Duration::from_secs(10));
    chain.add_block("TanqiLong => Jeffy: 1 BTC".to_string());
    println!("Produce blocking ...");

    println!("Start mining ...");
    thread::sleep(Duration::from_secs(10));
    chain.add_block("YaleXin => Jeffy: 2 BTC".to_string());
    println!("Produce blocking ...");

    for block in chain.blocks {
        println!("================================================");
        println!("{:#?}", block);
        println!("");
    }
}
