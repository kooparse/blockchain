mod blockchain;
mod helpers;
mod block;

use blockchain::Blockchain;

fn main() {
    let mut blockchain = Blockchain::new();
    blockchain.generate_block("new_block_1");
    blockchain.print_last_block();
}
