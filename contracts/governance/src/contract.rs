#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, to_binary};
use governance_types::errors::ContractError;
use governance_types::types::{ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg};
use crate::execute::{execute_vote, execute_whitelist};
use crate::queries::query_config;
use crate::state::{Config, store_config, read_config};


// Method is executed when a new contract instance is created. You can treat it as a constructor.
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> StdResult<Response> {
    let cfg = Config {
        admin : _info.sender.clone(),
        min_votes : _msg.min_votes,
        percentage : _msg.percentage,
        cur_votes : 0,
    };

    let result = store_config(deps.storage, &cfg);
    match result {
        Ok(x) => return Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("admin", &cfg.admin)),
        Err(x) => return Err(x) // Does this make sense? We'll see.
    };

}

// Methods which are executed when someone send call which changes blockchain state.
// It can be compared to Solidity NON view methods.
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        // TODO add required method types and handlers for each.
        ExecuteMsg::Whitelist { address, status } => execute_whitelist(deps, env, info, address, status),
        ExecuteMsg::Vote { vote } => execute_vote(deps, env, info, vote)
    }
}

// Methods which are executed when someone send a query (gas free call).
// It can be compared to Solidity view methods.
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> Result<Binary, ContractError> {
    match msg {
        // TODO implement missing even handlers
        QueryMsg::Config {} => {
            Ok(to_binary(&query_config(deps)?)?)
        }
        QueryMsg::GetVoter { .. } => {
            Ok(to_binary(&{})?)
        }
        QueryMsg::GetStatus { .. } => {
            Ok(to_binary(&{})?)
        }
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(
    _deps: DepsMut,
    _env: Env,
    _msg: MigrateMsg,
) -> StdResult<Response> {
    Ok(Response::default())
}
