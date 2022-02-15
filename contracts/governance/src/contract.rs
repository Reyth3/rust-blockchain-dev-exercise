#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, to_binary};
use governance_types::errors::ContractError;
use governance_types::types::{ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg};
use crate::execute::{execute_vote, execute_whitelist, execute_close};
use crate::queries::{query_config, query_status, query_voter};
use crate::state::{Config, store_config};


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
        ongoing : true,
        min_votes : _msg.min_votes,
        percentage : _msg.percentage,
        cur_votes : 0,
    };

    store_config(deps.storage, &cfg)?;

    return Ok(Response::new()
        .add_attribute("action", "instantiate")
        .add_attribute("admin", cfg.admin.as_str())
    );
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
        ExecuteMsg::Vote { vote } => execute_vote(deps, env, info, vote),
        ExecuteMsg::Close {} => execute_close(deps, env, info)
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
        QueryMsg::GetVoter { address } => {
            Ok(to_binary(&query_voter(deps, address)?)?)
        }
        QueryMsg::GetStatus { } => {
            Ok(to_binary(&query_status(deps)?)?)
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

/*#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info, MOCK_CONTRACT_ADDR};
    use cosmwasm_std::{attr, coins, CosmosMsg};

    #[test]
    fn proper_initialization() {
        let mut deps = mock_dependencies(&[]);

        let msg = InstantiateMsg {
            min_votes : 25,
            percentage: 50
        };
        let info = mock_info("creator", &coins(1, "BTC"));

        
        let res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
        assert_eq!(0, res.messages.len());

        assert_eq!(info.sender, res.) // TODO custom response for the instantiate call maybe

    }
}*/