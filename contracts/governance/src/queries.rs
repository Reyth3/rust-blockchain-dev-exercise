use cosmwasm_std::Deps;
use governance_types::errors::ContractError;
use governance_types::types::ConfigResponse;
use crate::state::read_config;

pub fn query_config(
    deps: Deps,
) -> Result<ConfigResponse, ContractError> {
    let cfg = read_config(deps.storage)?;
    let owner = cfg.admin.to_string();

    let resp = ConfigResponse {
        owner : owner, 
        ongoing : cfg.ongoing,
        min_votes : cfg.min_votes, 
        percentage : cfg.percentage, 
        cur_votes : cfg.cur_votes
    };

    Ok(resp)
}
