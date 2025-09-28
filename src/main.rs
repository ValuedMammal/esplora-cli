//! Mempool Space API Command Line Interface.
//!
//! This binary provides a command line interface (CLI) for
//! [`mempool_space_api`].

#![allow(unused_imports)]
#![allow(clippy::uninlined_format_args)]

use anyhow::anyhow;
use bitcoin::{address::NetworkUnchecked, consensus, Address, BlockHash, Transaction, Txid};
use clap::{Parser, Subcommand};
use mempool_space_api::{tokio, Http};
use mempool_space_api::{AsyncClient, Error, ReqwestClient, ReqwestError};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// CLI command.
    #[command(subcommand)]
    command: Commands,
    /// Server URL.
    #[clap(long, short, default_value = "https://mempool.space/api")]
    url: Option<String>,
}

#[derive(Subcommand)]
#[clap(rename_all = "lower")]
enum Commands {
    /// Get transaction by id.
    GetTx { txid: Txid },
    /// Get info of a transaction.
    GetTxInfo { txid: Txid },
    /// Get transaction at block index
    GetTxAtIndex { hash: BlockHash, index: usize },
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
    GetOutputStatus { txid: Txid, index: u32 },
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
    let url = cli.url.ok_or(anyhow!("must set esplora url"))?;
    let reqwest_client = ReqwestClient::default();
    let client = AsyncClient::new(&url, reqwest_client);

    match cli.command {
        Commands::GetTx { txid } => {
            let tx = client.get_tx(&txid).await?;
            println!("{:#?}", consensus::encode::serialize_hex(&tx));
        }
        Commands::GetTxInfo { txid } => {
            let res = client.get_tx_info(&txid).await?;
            println!("{:#?}", res);
        }
        Commands::GetTxAtIndex { hash, index } => {
            let txid = client.get_tx_at_index(&hash, index).await.map_err(raise_404)?;
            println!("{}", txid);
        }
        Commands::GetTxStatus { txid } => {
            let tx_status = client.get_tx_status(&txid).await?;
            println!("{:#?}", tx_status);
        }
        Commands::GetHeader { hash } => {
            let header = client.get_block_header(&hash).await?;
            println!("{:#?}", header);
        }
        Commands::GetBlockStatus { hash } => {
            let status = client.get_block_status(&hash).await?;
            println!("{:#?}", status);
        }
        Commands::GetBlock { hash } => {
            let block = client.get_block(&hash).await.map_err(raise_404)?;
            for tx in &block.txdata {
                println!("{:#?}", tx.compute_txid());
            }
        }
        Commands::GetMerkleProof { txid } => {
            let merkle_proof = client.get_merkle_proof(&txid).await?;
            println!("{:#?}", merkle_proof);
        }
        Commands::GetMerkleBlock { txid } => {
            let merkle_block = client.get_merkle_block(&txid).await?;
            println!("{:#?}", merkle_block);
        }
        Commands::GetOutputStatus { txid, index } => {
            let status = client.get_output_status(&txid, index).await?;
            println!("{:#?}", status);
        }
        Commands::Broadcast { tx_hex } => {
            let tx: Transaction = consensus::encode::deserialize_hex(&tx_hex)?;
            let txid = client.broadcast(&tx).await?;
            println!("{:#?}", txid);
        }
        Commands::GetTip => {
            let blocks = client.get_blocks(None).await?;
            println!("{:#?}", &blocks[0]);
        }
        Commands::GetBlockHash { height } => {
            let hash = client.get_block_hash(height).await.map_err(raise_404)?;
            println!("{:#?}", hash);
        }
        Commands::GetFeeEstimates => {
            let fees = client.get_recommended_fees().await?;
            println!("{:#?}", fees);
        }
        Commands::GetScriptHashTxs { address, last_seen } => {
            let addr = address.clone().require_network(bitcoin::Network::Bitcoin)?;
            let txs = client.get_scripthash_txs(&addr.script_pubkey(), last_seen).await?;
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

/// Return `Err(None)` if the response status is `404 NOT FOUND`.
fn raise_404(e: Error<ReqwestError>) -> anyhow::Error {
    if let Error::Http(ReqwestError::HttpResponse { status: 404, .. }) = e {
        anyhow!("None")
    } else {
        anyhow!(e)
    }
}
