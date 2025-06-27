//! Esplora API Command Line Interface.
//!
//! This binary provides a command line interface (CLI) for
//! [`rust-esplora-client`](esplora_client).

use std::str::FromStr;

use anyhow::anyhow;
use bitcoin::{Address, BlockHash, Txid};
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
    GetTx { txid: Txid },
    /// Get info of a transaction.
    GetTxInfo { txid: Txid },
    /// Get transaction at block index
    GetTxAtBlockIndex { hash: BlockHash, index: usize },
    /// Get transaction status by id
    GetTxStatus { txid: Txid },
    /// Get block header by block hash
    GetHeader { hash: BlockHash },
    /// Get block status by block hash
    GetBlockStatus { hash: BlockHash },
    /// Get block by block hash
    GetBlock { hash: BlockHash },
    /// Get transaction merkle proof by tx id
    GetMerkleProof { txid: Txid },
    /// Get transaction merkle block inclusion proof by id
    GetMerkleBlock { txid: Txid },
    /// Get output spending status by tx id and output index
    GetOutputStatus { txid: Txid, index: u64 },
    /// Broadcast transaction.
    Broadcast { tx_hex: String },
    /// Get blockchain tip height
    GetHeight,
    /// Get current blockchain tip block hash
    GetTipHash,
    /// Get block hash at height
    GetBlockHash { height: u32 },
    /// Get a fee estimate by confirmation target in sat/vB
    GetFeeEstimates,
    /// Get confirmed transaction history for the specified address/scripthash sorted by date
    GetScriptHashTxs {
        address: Address<bitcoin::address::NetworkUnchecked>,
        last_seen: Option<String>,
    },
    /// Get recent block summaries at the tip or at height if provided (max summaries is backend
    /// dependant).
    GetBlocks {
        /// Height to fetch blocks from.
        #[clap(long, short = 's')]
        height: Option<u32>,
    },
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let network = cli.network.expect("must set esplora url");
    let builder = Builder::new(&network);
    let client = builder.build_blocking();

    match cli.command {
        Commands::GetTx { txid } => {
            let tx = client.get_tx(&txid)?.ok_or(anyhow!("None"))?;
            println!("{:#?}", bitcoin::consensus::encode::serialize_hex(&tx));
        }
        Commands::GetTxInfo { txid } => {
            let tx_info = client.get_tx_info(&txid)?.ok_or(anyhow!("None"))?;
            println!("{:#?}", tx_info);
        }
        Commands::GetTxAtBlockIndex { hash, index } => {
            let txid = client
                .get_txid_at_block_index(&hash, index)?
                .ok_or(anyhow!("None"))?;
            println!("{:#?}", txid);
        }
        Commands::GetTxStatus { txid } => {
            let res = client.get_tx_status(&txid)?;
            println!("{:#?}", res);
        }
        Commands::GetHeader { hash } => {
            let res = client.get_header_by_hash(&hash)?;
            println!("{:#?}", res);
        }
        Commands::GetBlockStatus { hash } => {
            let res = client.get_block_status(&hash)?;
            println!("{:#?}", res);
        }
        Commands::GetBlock { hash } => {
            let block = client.get_block_by_hash(&hash)?.ok_or(anyhow!("None"))?;
            for tx in &block.txdata {
                if !tx.is_coinbase() {
                    println!("{:#?}", tx.compute_txid());
                }
            }
        }
        Commands::GetMerkleProof { txid } => {
            let res = client.get_merkle_proof(&txid)?;
            println!("{:#?}", res);
        }
        Commands::GetMerkleBlock { txid } => {
            let res = client.get_merkle_block(&txid)?;
            println!("{:#?}", res);
        }
        Commands::GetOutputStatus { txid, index } => {
            let res = client
                .get_output_status(&txid, index)?
                .ok_or(anyhow!("None"))?;
            println!("{:#?}", res);
        }
        Commands::Broadcast { tx_hex } => {
            let tx: bitcoin::Transaction = bitcoin::consensus::encode::deserialize_hex(&tx_hex)?;
            client.broadcast(&tx)?;
        }
        Commands::GetHeight => {
            let res = client.get_height()?;
            println!("{:#?}", res);
        }
        Commands::GetTipHash => {
            let res = client.get_tip_hash()?;
            println!("{:#?}", res);
        }
        Commands::GetBlockHash { height } => {
            let res = client.get_block_hash(height)?;
            println!("{:#?}", res);
        }
        Commands::GetFeeEstimates => {
            let res = client.get_fee_estimates()?;
            println!("{:#?}", res);
        }
        Commands::GetScriptHashTxs { address, last_seen } => {
            let mut last_txid = None;
            if let Some(s) = last_seen {
                last_txid = Some(Txid::from_str(&s)?);
            }
            let addr = address.clone().assume_checked();
            let txs = client.scripthash_txs(&addr.script_pubkey(), last_txid)?;
            for tx in txs {
                println!("{:#?}", tx.txid);
            }
        }
        Commands::GetBlocks { height } => {
            let res = client.get_blocks(height)?;
            println!("{:#?}", res);
        }
    }

    Ok(())
}
