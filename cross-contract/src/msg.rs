use cosmwasm_schema::{cw_serde, QueryResponses};

#[cw_serde]
pub struct InstantiateMsg {}

#[cw_serde]
pub enum ExecuteMsg {
    Update { address: String, message: String },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(GetOutput)]
    GetResponce { address: String },
    #[returns(GetOutput)]
    GetMessage {},
}

#[cw_serde]
pub struct GetOutput {
    pub message: String,
}

#[cw_serde]
pub struct UpdateHello {
    pub message: String,
}
