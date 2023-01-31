use crate::error::ContractError;
use crate::msg::{self, ExecuteMsg, GetOutput, InstantiateMsg, QueryMsg, UpdateHello};
use crate::state::{STATE, State};
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
use cosmwasm_std::{CosmosMsg, WasmMsg};
use cw2::set_contract_version;
use hello_world::*;


// version info for migration info
const CONTRACT_NAME: &str = "crates.io:cross-contract";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");


#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let state = State { address: "wasm1duhxhqme7aexqj7z5xee69u293x4c6udqu08t0hq6k7y69emlsfqfclze5".to_string(), message: "World".to_string() };
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    STATE.save(deps.storage, &state)?;
    Ok(Response::new())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Update { address, message } => update(deps, _info, (address, message)),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetResponce { address } => {
            let result = deps
                .querier
                .query_wasm_smart::<GetOutput>(address, &msg::QueryMsg::GetMessage {});
            to_binary(&result.unwrap())
        }
        QueryMsg::GetMessage {} => {
            let result = deps.querier.query_wasm_smart::<GetOutput>(
                "wasm1duhxhqme7aexqj7z5xee69u293x4c6udqu08t0hq6k7y69emlsfqfclze5",
                &msg::QueryMsg::GetMessage {},
            );
            to_binary(&result.unwrap())
        }
    }
}

fn update(
    deps: DepsMut,
    info: MessageInfo,

    msg: (String, String),
) -> Result<Response, ContractError> {

    STATE.update(deps.storage, |mut state| -> Result<_, ContractError> {
        state.address = format!("Hello {}", msg.0);
        state.message = msg.1.clone();
        Ok(state)
    })?;

    let action: CosmosMsg<_> = CosmosMsg::Wasm(WasmMsg::Execute {
        contract_addr: msg.0,
        msg: to_binary(&hello_world::msg::ExecuteMsg::Update{ message: msg.1 }).unwrap(),
        funds: info.funds,
    });
    Ok(Response::new()
        .add_message(action))
}

#[cfg(test)]
mod tests {}
