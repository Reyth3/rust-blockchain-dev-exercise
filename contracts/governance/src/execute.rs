use cosmwasm_std::{DepsMut, Env, MessageInfo, Response};
use governance_types::errors::ContractError;
use crate::state::{cast_vote,already_voted,read_config,store_config}; // I don't think I can actually do it like that, since this is a dependant package? But I don't have any other ideas

pub fn execute_vote(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    vote : i8,
) -> Result<Response, ContractError> {
    
    let addr = deps.api.addr_canonicalize(&info.sender.as_str())?;
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