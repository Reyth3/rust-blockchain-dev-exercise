use crate::state::count_votes;
use cosmwasm_std::Deps;
use governance_types::errors::ContractError;
use governance_types::types::{ConfigResponse, StatusResponse, Settlement, ResolvedResult,
    VoterResponse};
use crate::state::{read_config,already_voted,get_whitelist_status};

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

pub fn query_status(
    deps: Deps,
) -> Result<StatusResponse, ContractError> {
    let cfg = read_config(deps.storage)?;

    let mut resp = StatusResponse {
        settlement : Settlement::Ongoing {},
        votes : cfg.cur_votes,
    };

    if cfg.ongoing == false {
        let votes = count_votes(deps.storage)?;
        let mut result : Option<ResolvedResult> = None;
        let for_win = votes.for_count > votes.against_count;
        let tie = votes.for_count == votes.against_count;
        if for_win {
            result = Some(ResolvedResult::For);
        }
        else if tie {
            result = Some(ResolvedResult::Tie);
        }
        else { result = Some(ResolvedResult::Against); }
        
        resp.settlement = Settlement::Resolved {
            for_votes : votes.for_count,
            for_percentage : votes.for_percentage,
            against_votes : votes.against_count,
            against_percentage : votes.against_percentage,
            abstain_votes : votes.abstain_count,
            result : result.unwrap() // here unwrap() should be fine since we're handling else exhaustively up there
        }
    }

    Ok(resp)
}

pub fn query_voter(
    deps: Deps,
    address: String,
) -> Result<VoterResponse, ContractError> {
    let addr = deps.api.addr_canonicalize(address.as_str())?;
    Ok(VoterResponse {
        is_whitelisted : get_whitelist_status(deps.storage, addr.as_slice()),
        already_voted : already_voted(deps.storage, addr.as_slice()),
    })
}