# Hello World 

### Usage 
- Clone this repository
    ```
    https://github.com/shanithkk/cosmwasm-example.git
- Compile the hello world example
    ```
    RUSTFLAGS='-C link-arg=-s' cargo wasm
    ```
    or
    ```
    cargo build --release --lib --target wasm32-unknown-unknown
    ```
- Setup the environment for malaga 420 chain 
- After compilation of this package deploy the wasm to malaga 420
    ```
    wasmd tx wasm store hello_world.wasm --from <wallet_name> --node https://rpc.malaga-420.cosmwasm.com:443 --chain-id malaga-420 --gas-prices 0.05umlg --gas auto --gas-adjustment 1.3 -b block
    ```
- Copy the code id 
- Initiate the contract 
    ```
    wasmd tx wasm instantiate 3591 \
  '{ "message": "Shanith" }' \
  --from <wallet_name> --label "Group" --no-admin --node https://rpc.malaga-420.cosmwasm.com:443 --chain-id malaga-420 --gas-prices 0.05umlg --gas auto --gas-adjustment 1.3 -y -b block
  ```
- Aftre this cotract you will get the contract address, copy and save it for later use.
- Your can query the state of the contract or update the state using 
    ```
    wasmd tx wasm execute \
  <contract_address> \
  '{ "update": { "message": "Shanith K K"} }' \
  --from <wallet_name> --node https://rpc.malaga-420.cosmwasm.com:443 --chain-id malaga-420 --gas-prices 0.05umlg --gas auto --gas-adjustment 1.3
  ```
- Query the state
    ```
    wasmd query wasm contract-state smart \
  <contract_address> \
  '{ "get_message": {} }' --node https://rpc.malaga-420.cosmwasm.com:443 --chain-id malaga-420