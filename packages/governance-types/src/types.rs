use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    pub min_votes : u32,
    pub percentage : u8
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    Vote { vote : i8 },
    Whitelist { address : String, status : bool }, // 'status' will let un-whitelist in the future if needed
    Close {}, // The admin has an option to close the vote when the requirements are met
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    Config {},
    GetVoter { address: String },
    GetStatus {},
}

// We define a custom struct for each query response
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ConfigResponse {
    // TODO extend if needed
    pub owner: String,
    pub ongoing : bool,
    pub min_votes : u32,
    pub percentage : u8,
    pub cur_votes : u32,
}
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct VoterResponse {
    pub is_whitelisted : bool,
    pub already_voted : bool,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub enum ResolvedResult {
    For,
    Against,
    Tie
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub enum Settlement {
    Resolved {
        result : ResolvedResult,
        for_votes : u32,
        against_votes : u32,
        abstain_votes : u32,
        for_percentage : u8,
        against_percentage : u8,
    },
    Ongoing {},
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct StatusResponse {
    pub settlement : Settlement,
    pub votes : u32,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct MigrateMsg {}