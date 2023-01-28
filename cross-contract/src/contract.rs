#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
// use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{self, ExecuteMsg, GetOutput, InstantiateMsg, QueryMsg};

/*
// version info for migration info
const CONTRACT_NAME: &str = "crates.io:cross-contract";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");
*/

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    Ok(Response::new())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    Ok(Response::new())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetResponce { address } => {
            let result = deps.querier.query_wasm_smart::<GetOutput>(
                address,
                &msg::QueryMsg::GetMessage {},
            );
            to_binary(&result.unwrap())
        },
        QueryMsg::GetMessage {} => {
            let result = deps.querier.query_wasm_smart::<GetOutput>(
                "wasm1duhxhqme7aexqj7z5xee69u293x4c6udqu08t0hq6k7y69emlsfqfclze5",
                &msg::QueryMsg::GetMessage {},
            );
            to_binary(&result.unwrap())
        }
    }
}

#[cfg(test)]
mod tests {}
