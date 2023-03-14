pub mod models;
use models::block::Block;
use models::blockchain::Blockchain;
use models::transaction::Transaction;
use models::wallet::Wallet;

fn main() {
    let mut blockchain = Blockchain::new();
    let wallet1 = Wallet::new();
    let wallet2 = Wallet::new();
    let wallet3 = Wallet::new();
    println!("{:#?}{:#?}{:#?}", wallet1, wallet2, wallet3);

    let transaction1 = Transaction::new(&wallet3, &wallet1, 2.0);
    let block = Block::new(
        1,
        blockchain.chain.last().unwrap().hash.clone(),
        0,
        String::from(""),
        wallet3.public_key.clone(),
        None,
    );
    let block2 = Block::new(
        2,
        blockchain.chain.last().unwrap().hash.clone(),
        0,
        String::from(""),
        wallet3.public_key,
        None,
    );
    let block3 = Block::new(
        3,
        blockchain.chain.last().unwrap().hash.clone(),
        0,
        String::from(""),
        wallet2.public_key,
        Some(transaction1),
    );
    blockchain.add_block(block);
    blockchain.add_block(block2);
    blockchain.add_block(block3);

    println!("{:#?}", blockchain.wallet_balances);
}
