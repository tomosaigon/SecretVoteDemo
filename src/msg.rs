use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use cosmwasm_std::{Uint256, Timestamp };

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    Increment {},
    SubmitNetWorth {
        name: String,
        worth: u64,
    },
    Reset {},
    SubmitProposal {
        id: String,
        choice_count: u8,
        start_time: Timestamp,
        end_time: Timestamp,
    },
    RegisterProposalVoter {
        proposal_id: String,
        eth_addr: String,
        scrt_addr: String,
        power: Uint256,
    },
    CastVote {
        proposal_id: String,
        eth_addr: String,
        scrt_addr: String,
        choice: u8,
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    WhoIsRicher {},
    GetCount {},
    GetCountStatic {},
}

// We define a custom struct for each query response
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct CountResponse {
    pub count: i32,
}

/// We define a custom struct for each query response
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct RicherResponse {
    pub richer: String,
}
