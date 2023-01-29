# Cross Contract CosmWasm 
This is an example for cross contract call in cosmwasm blockchain.

### Usage
- Setup the Environment for Malaga 420
- Deploy the hello world contract and initialize it.
- Compile this contract and deploy 
- Initialize the contract
    ```
    wasmd tx wasm instantiate 3933 \
  '{ }' --from <wallet> --label "Group" --no-admin $TXFLAG -y -b block
  ```
- Copy the contract address
- Query the contract, the query will call the hello world contract and give the output
    
- There are two type of query iin this contract

    * Contract address hardcoded
        ``` 
        wasmd query wasm contract-state smart \
        <contract_address> \
        '{ "get_message": {} }' --node https://rpc.malaga-420.cosmwasm.com:443 --chain-id malaga-420
        ```
    * User given contract call
        ```
        wasmd query wasm contract-state smart \
        <contract_address> \
        '{ "get_responce": { "address": "wasm1duhxhqme7aexqj7z5xee69u293x4c6udqu08t0hq6k7y69emlsfqfclze5"} }' --node https://rpc.malaga-420.cosmwasm.com:443 --chain-id malaga-420
        ```

