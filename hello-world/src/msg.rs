use cosmwasm_schema::{cw_serde, QueryResponses};

#[cw_serde]
pub struct InstantiateMsg {
    pub message: String,
}

#[cw_serde]
pub enum ExecuteMsg {
    Update { message: String },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(GetOutput)]
    GetMessage {},
}

#[cw_serde]
pub struct GetOutput{
    pub message: String,
}
