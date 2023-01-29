# CosmWasm Contract Example
This repository contains example for cosmwasm contracts
### Setup Environment
- Install [wasmd](https://github.com/CosmWasm/wasmd) 
- Configure wasmd with malaga testnet
    Export these
    ```
    export CHAIN_ID="malaga-420"
    export TESTNET_NAME="malaga-420"
    export FEE_DENOM="umlg"
    export STAKE_DENOM="uand"
    export BECH32_HRP="wasm"
    export WASMD_VERSION="v0.27.0"
    export CONFIG_DIR=".wasmd"
    export BINARY="wasmd"

    export GENESIS_URL="https://raw.githubusercontent.com/CosmWasm/testnets/master/malaga-420/config/genesis.json"

    export RPC="https://rpc.malaga-420.cosmwasm.com:443"
    export FAUCET="https://faucet.malaga-420.cosmwasm.com"

    export COSMOVISOR_VERSION="v0.42.10"
    export COSMOVISOR_HOME=/root/.wasmd
    export COSMOVISOR_NAME=wasmd

    export NODE=(--node $RPC)
    export TXFLAG=($NODE --chain-id $CHAIN_ID --gas-prices 0.05umlg --gas auto --gas-adjustment 1.3)

- Create [wallet](https://book.cosmwasm.com/wasmd-quick-start/preparing-account.html)
- Add test token to the wllet 
    ```
    curl -X POST --header "Content-Type: application/json" \
  --data '{ "denom": "umlg", "address": "wasm1wukxp2kldxae36rgjz28umqtq792twtxdfe6ux" }' \
  https://faucet.malaga-420.cosmwasm.com/credit

### Example

- [Hello World](/hello-world/src/lib.rs) : Hello World smart contract example. 
- [Cross contract call](/cross-contract/src/lib.rs) : Cross contract example for cosmwasm.
