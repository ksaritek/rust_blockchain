use rust_blockchain::{
    convert_address, hash_pub_key, send_tx, utils, validate_address, Blockchain, Server,
    Transaction, UTXOSet, Wallets, ADDRESS_CHECK_SUM_LEN, CENTERAL_NODE, GLOBAL_CONFIG,
};
use data_encoding::HEXLOWER;
use log::LevelFilter;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "blockchain_rust")]
struct Opt {
    #[structopt(subcommand)]
    command: Command,
}

#[derive(StructOpt, Debug)]
enum Command {
    #[structopt(name = "createblockchain", about = "Create a new blockchain")]
    Createblockchain {
        #[structopt(name = "address", help = "The address to send genesis block reward to")]
        address: String,
    },
    #[structopt(name = "createwallet", about = "Create a new wallet")]
    Createwallet,
    #[structopt(
        name = "getbalance",
        about = "Get the wallet balance of the target address"
    )]
    GetBalance {
        #[structopt(name = "address", help = "The wallet address")]
        address: String,
    },
    #[structopt(name = "listaddresses", about = "Print local wallet addres")]
    ListAddresses,
    #[structopt(name = "send", about = "Add new block to chain")]
    Send {
        #[structopt(long = "from", help = "Source wallet address")]
        from: String,
        #[structopt(long = "to", help = "Destination wallet address")]
        to: String,
        #[structopt(long = "amount", help = "Amount to send")]
        amount: i32,
        #[structopt(long = "mine", help = "Mine immediately on the same node")]
        mine: bool,
    },
    #[structopt(name = "printchain", about = "Print blockchain all block")]
    Printchain,
    #[structopt(name = "reindexutxo", about = "rebuild UTXO index set")]
    Reindexutxo,
    #[structopt(name = "startnode", about = "Start a node with mining rewards going to a miner")]
    StartNode {
        #[structopt(long = "miner", help = "The miner address")]
        miner: Option<String>,
    },
}

fn main() {
    env_logger::builder().filter_level(LevelFilter::Info).init();
    let opt = Opt::from_args();
    match opt.command {
        Command::Createblockchain { address } => {
            let blockchain = Blockchain::create_blockchain(address.as_str());
            let utxo_set = UTXOSet::new(blockchain);
            utxo_set.reindex();
            println!("Done!");
        }
        Command::Createwallet => {
            let mut wallet = Wallets::new();
            let address = wallet.create_wallet();
            println!("Your new address: {}", address)
        }
        Command::GetBalance { address } => {
            let address_valid = validate_address(address.as_str());
            if address_valid == false {
                panic!("ERROR: Address is not valid")
            }
            let payload = utils::base58_decode(address.as_str());
            let pub_key_hash = &payload[1..payload.len() - ADDRESS_CHECK_SUM_LEN];

            let blockchain = Blockchain::new_blockchain();
            let utxo_set = UTXOSet::new(blockchain);
            let utxos = utxo_set.find_utxo(pub_key_hash);
            let mut balance = 0;
            for utxo in utxos {
                balance += utxo.get_value();
            }
            println!("Balance of {}: {}", address, balance);
        }
        Command::ListAddresses => {
            let wallets = Wallets::new();
            for address in wallets.get_addresses() {
                println!("{}", address)
            }
        }
        Command::Send {
            from,
            to,
            amount,
            mine,
        } => {
            if !validate_address(from.as_str()) {
                panic!("ERROR: Sender address is not valid")
            }
            if !validate_address(to.as_str()) {
                panic!("ERROR: Recipient address is not valid")
            }
            let blockchain = Blockchain::new_blockchain();
            let utxo_set = UTXOSet::new(blockchain.clone());
        
            let transaction =
                Transaction::new_utxo_transaction(from.as_str(), to.as_str(), amount, &utxo_set);

            if mine {
                let coinbase_tx = Transaction::new_coinbase_tx(from.as_str());
                let block = blockchain.mine_block(&vec![transaction, coinbase_tx]);
                utxo_set.update(&block);
            } else {
                send_tx(CENTERAL_NODE, &transaction);
            }
            println!("Success!")
        }
        Command::Printchain => {
            let mut block_iterator = Blockchain::new_blockchain().iterator();
            loop {
                let result = block_iterator.next();
                if let Some(block) = result {
                    println!("Pre block hash: {}", block.get_pre_block_hash());
                    println!("Cur block hash: {}", block.get_hash());
                    println!("Cur block Timestamp: {}", block.get_timestamp());
                    for tx in block.get_transactions() {
                        let cur_txid_hex = HEXLOWER.encode(tx.get_id());
                        println!("- Transaction txid_hex: {}", cur_txid_hex);

                        if tx.is_coinbase() == false {
                            for input in tx.get_vin() {
                                let txid_hex = HEXLOWER.encode(input.get_txid());
                                let pub_key_hash = hash_pub_key(input.get_pub_key());
                                let address = convert_address(pub_key_hash.as_slice());
                                println!(
                                    "-- Input txid = {}, vout = {}, from = {}",
                                    txid_hex,
                                    input.get_vout(),
                                    address,
                                )
                            }
                        }
                        for output in tx.get_vout() {
                            let pub_key_hash = output.get_pub_key_hash();
                            let address = convert_address(pub_key_hash);
                            println!("-- Output value = {}, to = {}", output.get_value(), address,)
                        }
                    }
                    println!()
                } else {
                    break;
                }
            }
        }
        Command::Reindexutxo => {
            let blockchain = Blockchain::new_blockchain();
            let utxo_set = UTXOSet::new(blockchain);
            utxo_set.reindex();
            let count = utxo_set.count_transactions();
            println!("Done! There are {} transactions in the UTXO set.", count);
        }
        Command::StartNode { miner } => {
            if let Some(addr) = miner {
                if validate_address(addr.as_str()) == false {
                    panic!("Wrong miner address!")
                }
                println!("Mining is on. Address to receive rewards: {}", addr);
                GLOBAL_CONFIG.set_mining_addr(addr);
            }
            let blockchain = Blockchain::new_blockchain();
            let sockert_addr = GLOBAL_CONFIG.get_node_addr();
            Server::new(blockchain).run(sockert_addr.as_str());
        }
    }
}
