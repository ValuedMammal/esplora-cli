//! Esplora API Command Line Interface.
//!
//! This binary provides a command line interface (CLI) for
//! [`rust-esplora-client`](esplora_client).

#![allow(clippy::uninlined_format_args)]

use anyhow::anyhow;
use bitcoin::{address::NetworkUnchecked, consensus, Address, BlockHash, Transaction, Txid};
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
#[clap(rename_all = "lower")]
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
    /// Get best blockhash and height
    GetTip,
    /// Get block hash at height
    GetBlockHash { height: u32 },
    /// Get a fee estimate by confirmation target in sat/vB
    GetFeeEstimates,
    /// Get confirmed transaction history for the specified address/scripthash sorted by date
    GetScriptHashTxs {
        address: Address<NetworkUnchecked>,
        last_seen: Option<Txid>,
    },
    /// Get recent block summaries at the tip or at height if provided (max summaries is backend
    /// dependent).
    GetBlocks {
        /// Height to fetch blocks from.
        #[clap(long, short = 's')]
        height: Option<u32>,
    },
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let network = cli.network.expect("must set esplora url");
    let builder = Builder::new(&network);
    let client = builder.build_async()?;

    match cli.command {
        Commands::GetTx { txid } => {
            let tx = client.get_tx(&txid).await?.ok_or(anyhow!("None"))?;
            println!("{:#?}", bitcoin::consensus::encode::serialize_hex(&tx));
        }
        Commands::GetTxInfo { txid } => {
            let res = client.get_tx_info(&txid).await?;
            println!("{:#?}", res);
        }
        Commands::GetTxAtBlockIndex { hash, index } => {
            let txid = client
                .get_txid_at_block_index(&hash, index)
                .await?
                .ok_or(anyhow!("None"))?;
            println!("{:#?}", txid);
        }
        Commands::GetTxStatus { txid } => {
            let tx_status = client.get_tx_status(&txid).await?;
            println!("{:#?}", tx_status);
        }
        Commands::GetHeader { hash } => {
            let header = client.get_header_by_hash(&hash).await?;
            println!("{:#?}", header);
        }
        Commands::GetBlockStatus { hash } => {
            let status = client.get_block_status(&hash).await?;
            println!("{:#?}", status);
        }
        Commands::GetBlock { hash } => {
            let block = client.get_block_by_hash(&hash).await?.ok_or(anyhow!("None"))?;
            for tx in &block.txdata {
                println!("{:#?}", tx.compute_txid());
            }
        }
        Commands::GetMerkleProof { txid } => {
            let res = client.get_merkle_proof(&txid).await?;
            println!("{:#?}", res);
        }
        Commands::GetMerkleBlock { txid } => {
            let res = client.get_merkle_block(&txid).await?;
            println!("{:#?}", res);
        }
        Commands::GetOutputStatus { txid, index } => {
            let status = client.get_output_status(&txid, index).await?.ok_or(anyhow!("None"))?;
            println!("{:#?}", status);
        }
        Commands::Broadcast { tx_hex } => {
            let tx: Transaction = consensus::encode::deserialize_hex(&tx_hex)?;
            client.broadcast(&tx).await?;
        }
        Commands::GetTip => {
            let blocks = client.get_blocks(None).await?;
            println!("{:#?}", &blocks[0]);
        }
        Commands::GetBlockHash { height } => {
            let hash = client.get_block_hash(height).await?;
            println!("{:#?}", hash);
        }
        Commands::GetFeeEstimates => {
            let fees = client.get_fee_estimates().await?;
            println!("{:#?}", fees);
        }
        Commands::GetScriptHashTxs { address, last_seen } => {
            let addr = address.clone().assume_checked();
            let txs = client.scripthash_txs(&addr.script_pubkey(), last_seen).await?;
            for tx in txs {
                println!("{:#?}", tx.txid);
            }
        }
        Commands::GetBlocks { height } => {
            let blocks = client.get_blocks(height).await?;
            println!("{:#?}", blocks);
        }
    }

    Ok(())
}
