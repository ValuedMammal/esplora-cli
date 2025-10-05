# Esplora-CLI 

A rust CLI for [esplora-client](https://github.com/bitcoindevkit/rust-esplora-client).

```
Bitcoin Esplora CLI API

Usage: esplora-cli [OPTIONS] <COMMAND>

Commands:
  gettx             Get transaction by id
  gettxinfo         Get info of a transaction
  gettxatindex      Get transaction at block index
  gettxstatus       Get transaction status by id
  getheader         Get block header by block hash
  getblockstatus    Get block status by block hash
  getblock          Get block by block hash
  getmerkleproof    Get transaction merkle proof by tx id
  getmerkleblock    Get transaction merkle block inclusion proof by id
  getoutputstatus   Get output spending status by tx id and output index
  broadcast         Broadcast transaction
  gettip            Get best blockhash and height
  getblockhash      Get block hash at height
  getfeeestimates   Get a fee estimate by confirmation target in sat/vB
  getscripthashtxs  Get confirmed transaction history for the specified address/scripthash sorted by date
  getblocks         Get recent block summaries at the tip or at height if provided (max summaries is backend dependent)
  help              Print this message or the help of the given subcommand(s)

Options:
  -n, --network <NETWORK>  [default: https://blockstream.info/api]
  -h, --help               Print help
  -V, --version            Print version
```
