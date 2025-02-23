use rust_blockchain::Wallets;
use log::LevelFilter;
use structopt::StructOpt;
use std::error::Error;

#[derive(Debug, StructOpt)]
#[structopt(name = "blockchain_rust")]
struct Opt {
    #[structopt(subcommand)]
    command: Command,
}

#[derive(StructOpt, Debug)]
enum Command {
    #[structopt(name = "createwallet", about = "Create a new wallet")]
    Createwallet,
    #[structopt(name = "listaddresses", about = "Print local wallet addres")]
    ListAddresses,
}

fn main() -> Result<(), Box<dyn Error>> {
    env_logger::builder().filter_level(LevelFilter::Info).init();

    let opt = Opt::from_args();
    match opt.command {
        Command::Createwallet => {
            let mut wallet = Wallets::new()?;
            let address = wallet.create_wallet()?;
            println!("Your new address: {}", address)
        }
        Command::ListAddresses => {
            let wallets = Wallets::new()?;
            for address in wallets.get_addresses() {
                println!("{}", address)
            }
        }
    }
    Ok(())
}
