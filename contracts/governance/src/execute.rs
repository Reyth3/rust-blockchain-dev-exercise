use cosmwasm_std::{DepsMut, Env, MessageInfo, Response};
use governance_types::errors::ContractError;
use crate::state::{cast_vote,already_voted,
    read_config,store_config,set_whitelist_status,get_whitelist_status,
    count_votes}; // I don't think I can actually do it like that, since this is a dependant package? But I don't have any other ideas
use std::cmp;

pub fn execute_vote(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    vote : i8,
) -> Result<Response, ContractError> {
    
    let addr = deps.api.addr_canonicalize(&info.sender.as_str())?;
    let whitelisted = get_whitelist_status(deps.storage, addr.as_slice());
    if whitelisted == false {
        return Err(ContractError::Unauthorized {});
    }

    let voted = already_voted(deps.storage, addr.as_slice());
    if voted == true {
        return Err(ContractError::AlreadyVoted { voter : info.sender })
    }
    
    cast_vote(deps.storage, addr.as_slice(), vote)?;
    
    let mut cfg = read_config(deps.storage)?;
    cfg.cur_votes+= 1;
    store_config(deps.storage, &cfg)?;

    return Ok(Response::new()
        .add_attribute("action", "execute vote")
        .add_attribute("voter", info.sender.as_str())
        .add_attribute("vote", vote.to_string())
    );
}

pub fn execute_whitelist(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    address : String,
    status : bool
) -> Result<Response, ContractError> {
    
    let cfg = read_config(deps.storage)?;
    if info.sender != cfg.admin {
        return Err(ContractError::Unauthorized {});
    }

    let addr = deps.api.addr_canonicalize(info.sender.as_str())?;
        
    set_whitelist_status(deps.storage, addr.as_slice(), status)?;


    return Ok(Response::new()
        .add_attribute("action", "execute whitelist")
        .add_attribute("address", address)
        .add_attribute("status", status.to_string())
    );
}

pub fn execute_close(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
) -> Result<Response, ContractError> {
    
    let mut cfg = read_config(deps.storage)?;
    if info.sender != cfg.admin { // Only admin can close the voting
        return Err(ContractError::Unauthorized {});
    }

    if cfg.cur_votes < cfg.min_votes { // The min votes requirements has to be met
        return Err(ContractError::NotFulfilled {});
    }

    let votes = count_votes(deps.storage)?;
    let highest_percentage = cmp::max(votes.for_percentage, votes.against_percentage);
    if highest_percentage < cfg.percentage { // The percentage requirement not met
        return Err(ContractError::NotFulfilled {});
    }
        
    cfg.ongoing = false;
    store_config(deps.storage, &cfg)?;

    return Ok(Response::new()
        .add_attribute("action", "execute close")
    );
}