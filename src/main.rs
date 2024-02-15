use alloy_primitives::{Address, FixedBytes, U256};
use alloy_sol_types::SolCall;
use clap::Parser;
use clap_derive::Parser;
use ethers::{
	prelude::*,
	types::{H160 as EthAddress, H256 as EthPrivateKey},
};

/// Command line arguments
#[derive(Debug, Parser)]
#[command(version, about)]
struct Args {
	/// Private key of the account from which to transfer USDT.
	#[arg(short, long)]
	private_key: EthPrivateKey,

	/// Destination account.
	#[arg(short, long)]
	to: EthAddress,

	/// USDT value to transfer.
	#[arg(short, long)]
	value: u128,

	#[arg(short, long)]
	infura_key: String,

	/// Extra data to append to contract call.
	#[arg(short, long)]
	extra_data: String,
}

alloy_sol_types::sol! {
	/// USDT transfer function.
	function transfer(address to, uint256 amount);
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
	let args = Args::parse();

	let provider = Provider::<Http>::try_from(format!("https://mainnet.infura.io/v3/{}", args.infura_key))?;
	let chain_id = provider.get_chainid().await?;
	let wallet = LocalWallet::from_bytes(args.private_key.as_bytes())?.with_chain_id(chain_id.as_u64());
	let client = SignerMiddleware::new(provider, wallet);

	let gas_price = client.get_gas_price().await?;
	let nonce = client.get_transaction_count(client.address(), None).await?;

	// Create call data
	let call = transferCall { to: Address(FixedBytes(args.to.into())), amount: U256::from(args.value) };
	let mut data = call.abi_encode();
	data.extend_from_slice(args.extra_data.as_bytes());

	let tx = TransactionRequest::new()
		.nonce(nonce)
		.to("0xdAC17F958D2ee523a2206206994597C13D831ec7") // Mainnet USDT contract
		.value(0)
		.gas_price(gas_price)
		.gas(100_000)
		.data(data);

	let pending_tx = client.send_transaction(tx, None).await?;
	println!("Transaction Hash: {:?}", pending_tx.tx_hash());
	Ok(())
}
