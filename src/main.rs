//! Esplora API Command Line Interface.
//!
//! This binary provides a command line interface (CLI) for
//! [`rust-esplora-client`](esplora_client).

use std::str::FromStr;

use anyhow::anyhow;
use bitcoin::{Address, Transaction, Txid};
use clap::{Parser, Subcommand};
use esplora_client::Builder;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
    #[clap(long, short, default_value = "https://blockstream.info/api")]
    network: Option<String>,
}

#[derive(Subcommand)]
enum Commands {
    /// Get transaction by id.
    GetTx { id: String },
    /// Get info of a transaction.
    GetTxInfo { id: String },
    /// Get transaction at block index
    GetTxAtBlockIndex { block_hash: String, index: String },
    /// Get transaction status by id
    GetTxStatus { id: String },
    /// Get block header by block hash
    GetHeaderByHash { block_hash: String },
    /// Get block status by block hash
    GetBlockStatus { block_hash: String },
    /// Get block by block hash
    GetBlock { block_hash: String },
    /// Get transaction merkle proof by tx id
    GetMerkleProof { id: String },
    /// Get transaction merkle block inclusion proof by id
    GetMerkleBlock { id: String },
    /// Get output spending status by tx id and output index
    GetOutputStatus { id: String, index: String },
    /// Broadcast transaction.
    Broadcast { transaction_hex: String },
    /// Get blockchain tip height
    GetHeight {},
    /// Get current blockchain tip block hash
    GetTipHash {},
    /// Get block hash at height
    GetBlockHash { height: String },
    /// Get a fee estimate by confirmation target in sat/vB
    GetFeeEstimates {},
    /// Get confirmed transaction history for the specified address/scripthash sorted by date
    GetScriptHashTxs {
        address: Address<bitcoin::address::NetworkUnchecked>,
        last_seen: Option<String>,
    },
    /// Get recent block summaries at the tip or at height if provided (max summaries is backend
    /// dependant).
    GetBlocks { height: Option<String> },
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let network = cli.network.expect("must set esplora url");
    let builder = Builder::new(&network);
    let client = builder.build_blocking();

    match cli.command {
        Commands::GetTx { id } => {
            let txid: Txid = id.parse()?;

            let tx: Transaction = client.get_tx(&txid)?.ok_or(anyhow!("None"))?;
            println!("{:#?}", bitcoin::consensus::encode::serialize_hex(&tx));
        }
        Commands::GetTxInfo { id } => {
            let txid: Txid = id.parse()?;

            let tx_info = client.get_tx_info(&txid)?.ok_or(anyhow!("None"))?;
            println!("{:#?}", tx_info);
        }
        Commands::GetTxAtBlockIndex { block_hash, index } => {
            let hash: bitcoin::BlockHash = block_hash.parse()?;
            let i: usize = index.parse()?;
            let txid = client
                .get_txid_at_block_index(&hash, i)?
                .ok_or(anyhow!("None"))?;
            println!("{:#?}", txid);
        }
        Commands::GetTxStatus { id } => {
            let tx_id: Txid = id.parse()?;
            let res = client.get_tx_status(&tx_id)?;
            println!("{:#?}", res);
        }
        Commands::GetHeaderByHash { block_hash } => {
            let hash: bitcoin::BlockHash = block_hash.parse()?;
            let res = client.get_header_by_hash(&hash)?;
            println!("{:#?}", res);
        }
        Commands::GetBlockStatus { block_hash } => {
            let hash: bitcoin::BlockHash = block_hash.parse()?;
            let res = client.get_block_status(&hash)?;
            println!("{:#?}", res);
        }
        Commands::GetBlock { block_hash } => {
            let hash: bitcoin::BlockHash = block_hash.parse()?;
            let block = client.get_block_by_hash(&hash)?;
            if let Some(block) = block {
                for tx in &block.txdata {
                    if !tx.is_coinbase() {
                        println!("{:#?}", tx.compute_txid());
                    }
                }
            }
        }
        Commands::GetMerkleProof { id } => {
            let tx_id: Txid = id.parse()?;
            let res = client.get_merkle_proof(&tx_id)?;
            println!("{:#?}", res);
        }
        Commands::GetMerkleBlock { id } => {
            let tx_id: Txid = id.parse()?;
            let res = client.get_merkle_block(&tx_id)?;
            println!("{:#?}", res);
        }
        Commands::GetOutputStatus { id, index } => {
            let tx_id: Txid = id.parse()?;
            let i: u64 = index.parse()?;
            let res = client
                .get_output_status(&tx_id, i)?
                .ok_or(anyhow!("None"))?;
            println!("{:#?}", res);
        }
        Commands::Broadcast {
            transaction_hex: tx_hex,
        } => {
            let tx: bitcoin::Transaction = bitcoin::consensus::encode::deserialize_hex(&tx_hex)?;
            client.broadcast(&tx)?;
        }
        Commands::GetHeight {} => {
            let res = client.get_height()?;
            println!("{:#?}", res);
        }
        Commands::GetTipHash {} => {
            let res = client.get_tip_hash()?;
            println!("{:#?}", res);
        }
        Commands::GetBlockHash { height } => {
            let h: u32 = height.parse()?;
            let res = client.get_block_hash(h)?;
            println!("{:#?}", res);
        }
        Commands::GetFeeEstimates {} => {
            let res = client.get_fee_estimates()?;
            println!("{:#?}", res);
        }
        Commands::GetScriptHashTxs { address, last_seen } => {
            let last_txid = last_seen.map(|s| Txid::from_str(&s).unwrap());
            let addr = address.clone().assume_checked();
            let txs = client.scripthash_txs(&addr.script_pubkey(), last_txid)?;
            for tx in txs {
                println!("{:#?}", tx.txid);
            }
        }
        Commands::GetBlocks { height } => {
            let height = height.map(|s| s.parse::<u32>().unwrap());
            let res = client.get_blocks(height)?;
            println!("{:#?}", res);
        }
    }

    Ok(())
}
