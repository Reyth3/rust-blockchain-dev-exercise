use cosmwasm_std::Deps;
use governance_types::errors::ContractError;
use governance_types::types::ConfigResponse;
use crate::state::read_config;

pub fn query_config(
    deps: Deps,
) -> Result<ConfigResponse, ContractError> {
    let owner = String::from("");

    let resp = ConfigResponse {
        owner
    };

    Ok(resp)
}
